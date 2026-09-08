# GitHub-based sync for orivo (replaces the Google Drive backup)

## Context

orivo previously backed up its local SQLite database to Google Drive via a self-hosted OAuth app (`src/utils/drive/`, `src/domains/backup.rs`, and the `login`/`logout`/`status`/`backup`/`restore` commands). That required a maintainer-registered Google Cloud OAuth client, a `GOOGLE_CLIENT_ID`/`GOOGLE_CLIENT_SECRET` pair baked into release builds, and an OS keyring entry per user — a lot of moving parts for a single-file backup.

This has been removed entirely and replaced with a GitHub-based sync that piggybacks on the GitHub CLI (`gh`), which most users already have installed and authenticated: no OAuth client to register, no secrets to embed, no keyring — `gh auth login` handles authentication outside of orivo.

## Design

**Data synced:** just the SQLite database (`db_path()`), matching the old backup's scope — it's the single source of all user data (todos + pomodoro sessions). Config (`config.toml`) and UI state (`store.json`) are per-machine preferences, not synced.

**Repo:** a private repo named `orivo-data` under the signed-in `gh` user's account, created on first sync via `gh repo create owner/orivo-data --private` if it doesn't already exist.

**Local working copy:** a real git clone of that repo at `sync_dir()` (`<state dir>/sync`), holding one file, `orivo.sqlite.gz` (gzip-compressed, mirroring the old backup's compression).

**History:** each sync amends and force-pushes the same single commit rather than creating a new one, so the repo doesn't grow forever from repeated full-file snapshots — the repo is a single overwritten snapshot, not a version history, matching WhatsApp/Drive-style backup semantics.

**Change detection:** a small local `SyncState` (`sync_state_path()`, JSON) stores the SHA-256 hash of the database as of the last successful sync — the same skip-if-unchanged approach the old backup used, extended to both directions:

```rust
struct SyncState {
    last_synced_hash: Option<String>,
}
```

On `orivo sync`:
1. Ensure `gh` is signed in (`gh auth status`); error out with instructions if not.
2. `gh::ensure_repo()` — create `owner/orivo-data` (private) if missing.
3. `repo::ensure_clone()` — `gh repo clone` into `sync_dir()` if not already cloned, else `git fetch origin`.
4. Compute `local_hash` (hash of gzipped local db) and `remote_hash` (hash of `orivo.sqlite.gz` as committed on `origin/<branch>`, via `git show`, or `None` if nothing's been pushed yet).
5. Compare both against `last_synced_hash`:
   - neither changed → "already up to date"
   - only local changed → push (write file, commit --amend or first commit, `push --force-with-lease`)
   - only remote changed → pull (download via `git show`, gunzip, atomically overwrite `db_path()`)
   - both changed → prompt: keep local (push) or keep remote (pull)
6. Update and save `SyncState.last_synced_hash`.

### New modules
- `src/utils/sync/gh.rs` — `gh` CLI wrapper: `is_authenticated()`, `ensure_repo()`.
- `src/utils/sync/repo.rs` — `git` CLI wrapper for the local clone: `ensure_clone()`, `current_branch()`, `read_remote_file()`, `commit_and_push()`.
- `src/domains/sync.rs` — orchestration: `SyncState` load/save, `run_sync()`, gzip/gunzip/hash helpers (carried over from the old backup domain).
- `src/cmds/sync.rs` — thin `Cmd` wrapper, replacing `backup`/`restore`/`login`/`logout`/`status`.

### Removed
- `src/utils/drive/` (OAuth2 device/browser flow, Drive REST client)
- `src/domains/backup.rs`, `src/cmds/{backup,login,logout,restore,status}.rs`
- `oauth2`, `keyring`, `reqwest`, `dotenvy` dependencies (no longer needed — `gh`/`git` are shelled out to instead of calling APIs directly, and there's no `.env`-provided secret left to load)
- `GOOGLE_CLIENT_ID`/`GOOGLE_CLIENT_SECRET` from CI (`build.yml`), `Cross.toml` passthrough, and `.env.example`

### Kept
- `flate2` (gzip), `sha2` (SHA-256 change detection) — same helpers, reused in `domains/sync.rs`.

## Verification
- `cargo build` / `cargo clippy` — confirm the new modules compile cleanly.
- Manual: run `orivo sync` with `gh` signed in and no `orivo-data` repo yet — confirm the repo is created private, and the database is pushed.
- Run `orivo sync` again immediately — confirm "already up to date", no network calls.
- Edit a todo, run `orivo sync` — confirm it pushes (repo still shows one commit, amended).
- On a second machine (or after deleting the local `sync_dir`/db), run `orivo sync` — confirm it pulls and restores the data.
- Modify the local db *and* push a change from another clone before syncing — confirm the conflict prompt appears and both choices work.
