<p>
  <img src="orivo.svg" width="120" alt="orivo logo"/>
</p>

# orivo

[![License](https://img.shields.io/crates/l/orivo)](https://github.com/mt-shihab26/orivo/blob/main/LICENSE)
[![Build](https://github.com/mt-shihab26/orivo/actions/workflows/build.yml/badge.svg)](https://github.com/mt-shihab26/orivo/actions/workflows/build.yml)
[![Tests](https://github.com/mt-shihab26/orivo/actions/workflows/test.yml/badge.svg)](https://github.com/mt-shihab26/orivo/actions/workflows/test.yml)
[![Crates.io](https://img.shields.io/crates/v/orivo)](https://crates.io/crates/orivo)
[![docs.rs](https://img.shields.io/docsrs/orivo)](https://docs.rs/orivo)

A terminal-based (TUI) Todos + Pomodoro timer written in [Rust](https://www.rust-lang.org)

## Installation

### [Omarchy](https://omarchy.org)

<!-- ```sh -->
<!-- omarchy pkg add orivo -->
<!-- ``` -->

```sh
git clone --depth 1 https://github.com/mt-shihab26/orivo.git /tmp/orivo
cd /tmp/orivo
makepkg -si
```

Installs the AUR package (see [`PKGBUILD`](PKGBUILD)), which places `orivo` on `/usr/bin` and registers a desktop entry that launches it via `omarchy-launch-terminal` — so it opens in whatever terminal you've configured as default.

Usage (app launcher):

1. Press `SUPER + ALT + SPACE` to open the app launcher
2. Search for orivo

Usage (terminal):

```sh
$ orivo
```

Uninstall:

```sh
omarchy pkg drop orivo
```

### Cargo (any OS, builds from source)

**Requires sqlite3** — install it for your OS and ensure it's on your `PATH` before building.

```sh
$ cargo install orivo
```

Run from any terminal:
```sh
$ orivo
```

## Configuration

Config file location: `~/.config/orivo/config.toml`

```toml
# Orivo configuration


show_fps = false # show the FPS counter in the TUI header on startup


# Pomodoro timer settings — controls session lengths and when long breaks are triggered.
[timer]
show_millis         = false   # show milliseconds in the timer display
work_duration       = 25      # work session length in minutes         (min: 1, max: 120)
break_duration      = 5       # short break length in minutes          (min: 1, max: 60)
long_break_duration = 15      # long break length in minutes           (min: 1, max: 60)
long_break_interval = 4       # work sessions before a long break      (min: 1, max: 10)
daily_session_goal  = 16      # target work sessions to complete today (min: 1, max: 24)

```

### Root Options

- `show_fps` → show the FPS counter when the TUI starts. You can still toggle it at runtime with `Ctrl+F`.

### Timer (`[timer]`)

The [Pomodoro technique](https://en.wikipedia.org/wiki/Pomodoro_Technique) breaks work into focused sessions separated by breaks:

- **Work session** → focused work period (default: 25 min)
- **Short break** → rest between sessions (default: 5 min)
- **Long break** → rest after completing a full cycle (default: 15 min)

A full cycle = `work_duration` × `long_break_interval` work sessions. After that many sessions, a long break is triggered instead of a short one.

```
work → break → work → break → work → break → work → LONG BREAK  (cycle of 4)
```

**Daily session goal** (`daily_session_goal`) sets how many work sessions you aim to complete each day. Progress is shown as a session tracker in the timer tab. Once the goal is reached, the tracker fills completely.

- Default: `16` sessions
- Range: `1` – `24` sessions

## Backup & Restore

orivo can back up its local database to your Google Drive, WhatsApp-style: a single gzip-compressed snapshot stored in Drive's hidden **app-data folder** (invisible in the normal Drive UI), overwritten in place on every backup rather than piling up copies.

```sh
$ orivo login    # sign in to Google Drive and save credentials for later backup/restore
$ orivo backup   # upload the current database, skipped if nothing changed since the last backup
$ orivo restore  # download the backup and overwrite the local database (destructive, asks for confirmation)
```

The first `orivo login` (or `orivo backup` / `orivo restore`, which trigger sign-in automatically if needed) starts a one-time Google sign-in using the OAuth **device code** flow: the terminal prints a URL and a short code, which you open and enter on any browser (phone, another computer, etc.) to grant access. After that, the resulting refresh token is stored in your OS's secret service (gnome-keyring/kwallet on Linux) and reused automatically — no browser needed again unless access is revoked.

Only the `drive.appdata` scope is requested, so orivo can never see or touch the rest of your Drive.

### For maintainers: one-time OAuth client setup

Backup/restore requires a Google Cloud OAuth client of type **"TVs and Limited Input devices"**, registered once in the [Google Cloud Console](https://console.cloud.google.com/apis/credentials) to obtain a client ID and its accompanying public client secret. This secret isn't confidential for this client type — the same model `gh`/`docker`/`gcloud` use — but we still don't commit the literal value to git, since automated secret scanners (GitHub push protection, gitleaks, etc.) can't tell a "public by design" OAuth secret from a leaked one and will flag/block on it regardless.

Instead, the value is baked into the binary at **build time** via environment variables, so `git blame`/history never contains it:

```sh
$ ORIVO_GOOGLE_CLIENT_ID=xxx.apps.googleusercontent.com \
  ORIVO_GOOGLE_CLIENT_SECRET=yyy \
  cargo build --release
```

Building without these set still succeeds — `login`/`backup`/`restore` just fail at runtime with a clear "orivo was built without ORIVO_GOOGLE_CLIENT_ID set" error, so regular contributors working on unrelated features don't need Google credentials to build the project. Whoever cuts release binaries (CI, AUR packaging, etc.) needs to supply both as secrets in that build environment.
