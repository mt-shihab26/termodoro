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

orivo can back up its local database to Google Drive, WhatsApp-style: a single gzip-compressed snapshot in Drive's hidden app-data folder, overwritten in place each time.

```sh
$ orivo login    # sign in to Google Drive
$ orivo backup   # upload the current database (skipped if nothing changed)
$ orivo restore  # download the backup and overwrite the local database (asks for confirmation)
```

The first sign-in uses Google's OAuth **device code** flow: orivo prints a URL and a short code, you approve it on any browser, and the resulting token is stored in your OS keyring — no browser needed again after that. Only the `drive.appdata` scope is requested, so orivo never sees the rest of your Drive.

## Development

```sh
$ git clone https://github.com/mt-shihab26/orivo.git
$ cd orivo
```

To work on backup/restore locally, you need a Google OAuth client:

1. In the [Google Cloud Console](https://console.cloud.google.com), create a project (or select an existing one).
2. Go to **APIs & Services → Library**, search for **Google Drive API**, and enable it.
3. Go to **APIs & Services → OAuth consent screen**: choose **External**, fill in the required fields (app name, support email), and under **Test users** add your own Google account — the app stays unverified during development, so only listed test users can sign in.
4. Go to **APIs & Services → Credentials → Create Credentials → OAuth client ID**, and choose application type **"TVs and Limited Input devices"**. This gives you a **Client ID** and **Client secret**.
5. Copy `.env.example` to `.env` and fill in `GOOGLE_CLIENT_ID` / `GOOGLE_CLIENT_SECRET` with those two values.

No extra scope configuration is needed on the Google Cloud side — orivo only ever requests `https://www.googleapis.com/auth/drive.appdata` at sign-in time (see `src/utils/drive/auth.rs`), which is Drive's restricted per-app storage scope, not general Drive access.

CI sets the same two variables as real environment variables from GitHub Actions secrets. Without either set, everything still builds and runs — `login`/`backup`/`restore` just fail with a clear error.

```sh
$ cargo run
```

Debug builds keep config, database, and log files under `./local/` instead of the real system paths, so you can inspect or wipe local state freely.

```sh
$ cargo run seed    # reset the local database and fill it with sample todos/sessions
$ cargo test        # run the test suite
$ cargo fmt          # format code
$ cargo clippy       # lint
```
