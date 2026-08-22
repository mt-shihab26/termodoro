# Google Drive backup for orivo (WhatsApp-style)

## Context

orivo is a Rust terminal (TUI) Todos + Pomodoro app. All user data lives in a single local SQLite file (`~/.local/state/orivo/orivo.sqlite`, opened via SeaORM — see `src/utils/db.rs`, `src/utils/path.rs::db_path()`). There is currently **no backup/export/sync feature at all**: a `[db]` Turso config was scaffolded but never wired to anything and was just removed (commit `cf614e7`) as dead code. There's also no existing OAuth/HTTP client dependency in `Cargo.toml`.

The goal is a WhatsApp-like backup: the app periodically snapshots its data to the user's Google Drive so it survives a reinstall/new machine, without cluttering Drive or re-uploading unchanged data on every run. Per the answers given: use OAuth **device-code flow** (no browser embedding possible in a terminal app) and ship a **manual `orivo backup` / `orivo restore` command** for v1 (no silent auto-backup yet).

**Format decision (the core question):** back up the raw SQLite file itself, gzip-compressed, as a single fixed-name file stored in Google Drive's hidden **`appDataFolder`** (a per-app storage area invisible in the user's normal Drive UI — exactly analogous to where WhatsApp/other apps stash their backup). Two things prevent duplication:
- **No duplicate files in Drive:** the app persists the Drive `fileId` of the backup locally after the first upload, and every subsequent backup calls `files.update` (overwrite-in-place) on that same `fileId` instead of `files.create`. There is only ever one backup file.
- **No redundant uploads:** before uploading, compute a SHA-256 hash of the compressed bytes and compare it to the hash stored from the last successful backup. If unchanged, skip the network call entirely.

This mirrors WhatsApp's own model (single full snapshot, overwritten each time) rather than attempting binary diffing, which would be complex for little benefit at this data size (todos + pomodoro sessions).

## Design

### New dependencies (`Cargo.toml`)
- `reqwest` (default-features = false, features = `json`, `multipart`, `rustls-tls` — consistent with the existing `runtime-tokio-rustls` choice) — Drive REST calls.
- `oauth2` — implements the OAuth2 Device Authorization Grant against Google's endpoints, plus refresh-token exchange.
- `keyring` — stores the Google refresh token in the OS Secret Service (gnome-keyring/kwallet), never in a plaintext file. Matches the Linux-only build matrix.
- `flate2` — gzip compression of the sqlite file.
- `sha2` — content hashing for the skip-if-unchanged check.

### New module: `src/utils/drive/`
- `auth.rs` — device-code login flow: request a device/user code from Google, print the verification URL + code via `println!` (reusing the "plain stdout" pattern already used by `seed`/`version`), poll for the token, then store the refresh token via `keyring::Entry`. Also exposes a `token()` helper that loads the refresh token and exchanges it for a fresh short-lived access token before each API call (access tokens are never persisted).
- `client.rs` — thin Drive API wrapper: `find_backup_file()` (list files in `appDataFolder` by fixed name, used to recover a lost local `fileId`), `upload_or_update(bytes)` (create if no known `fileId`, else `files.update`), `download(file_id)`.
- Scope used: `https://www.googleapis.com/auth/drive.appdata` (limited, doesn't require a CASA security review, only a basic OAuth consent screen).

### New local state file
Extend the existing `Store`-style pattern (`src/utils/store.rs` is the precedent) with a small `BackupState` struct, persisted at a new path added to `src/utils/path.rs` (e.g. `backup_state_path()` → `~/.local/state/orivo/backup.json`):
```rust
struct BackupState {
    drive_file_id: Option<String>,
    last_backup_hash: Option<String>,
    last_backup_at: Option<OffsetDateTime>,
}
```

### New domain logic: `src/domains/backup.rs`
Following the existing `cmds/*.rs` (thin CLI wrapper) → `domains/*.rs` (actual logic) split seen in `seed`:
- `run_backup()`: ensure authenticated (trigger device flow if no stored refresh token) → read `db_path()` → gzip it → hash it → compare to `BackupState.last_backup_hash`; if equal, print "nothing changed, skipping" and return → otherwise `upload_or_update`, using/recovering `drive_file_id` as described above → update and save `BackupState`.
- `run_restore()`: ensure authenticated → resolve `drive_file_id` (from state, or `find_backup_file()` as fallback) → download + gunzip → prompt for confirmation via `std::io::stdin().read_line` (new pattern, nothing to reuse — note this in the command's help text since it's destructive) → write to a temp file and atomically rename over `db_path()` (avoid a partial/corrupt DB on failure) → done.

Async: both wrap their Drive/HTTP calls the same way `db.rs` already bridges SeaORM's async calls into the sync `Cmd::run` — via `orivo::utils::db::rt().block_on(async { ... })`.

### CLI wiring
- `src/cmds/backup.rs`, `src/cmds/restore.rs` — new `Cmd` impls mirroring `src/cmds/seed.rs`'s shape (`help()` + `run(self: Box<Self>)`), delegating to `domains::backup::run_backup()` / `run_restore()`.
- `src/main.rs` — add `Some("backup")` and `Some("restore")` match arms next to the existing `"seed"`/`"version"` arms, and register their `help()` fns in the `help()` function's `helps` array (both the debug and release variants, since unlike `seed` these should ship in release builds).
- `src/cmds/mod.rs` — add `pub mod backup;` / `pub mod restore;`.

### Prerequisite (one-time, outside the codebase)
A Google Cloud OAuth client of type "TVs and Limited Input devices" needs to be registered once (by the project maintainer) to get a client ID (+ the accompanying public "secret", which for this installed-app client type is not actually confidential) to embed as a constant in the binary — the same approach tools like `gh`/`docker` use for a public CLI client. This is a manual Google Cloud Console step, not something to script.

## Files to touch
- `Cargo.toml` — add the 5 dependencies above
- `src/utils/path.rs` — add `backup_state_path()`
- `src/utils/drive/mod.rs`, `auth.rs`, `client.rs` — new
- `src/domains/backup.rs` — new (backup + restore logic, `BackupState` load/save)
- `src/cmds/backup.rs`, `src/cmds/restore.rs` — new
- `src/cmds/mod.rs` — register new modules
- `src/main.rs` — new match arms + help registration
- `README.md` — document the new commands and the one-time Google auth step

## Verification
- `cargo build` / `cargo clippy` to confirm the new modules compile cleanly against the existing `Cmd` trait and async-bridging pattern.
- Manual end-to-end test: run `orivo backup` in a debug build (uses `./local/orivo.sqlite`) — confirm the device-code prompt appears, complete login in a browser, confirm a file appears in Drive's hidden app-data area (verifiable via the Drive API's `files.list(spaces='appDataFolder')`, since it won't show in the normal Drive UI), confirm the local `backup.json` records a `drive_file_id` and hash.
- Run `orivo backup` again immediately with no data changes — confirm it prints the "nothing changed" skip message and makes no network call (verify via logs).
- Modify a todo, run `orivo backup` again — confirm it updates the *same* `fileId` (check Drive shows one file, not two).
- Delete/rename the local sqlite file, run `orivo restore` — confirm the confirmation prompt, then confirm the DB is restored and the app opens with the expected todos/sessions.
