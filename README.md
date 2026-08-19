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

### Omarchy

<!-- ```sh -->
<!-- omarchy pkg add orivo -->
<!-- ``` -->

```sh
git clone https://github.com/mt-shihab26/orivo.git /tmp/orivo
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


# Database connection — Orivo uses Turso (libSQL/SQLite) for syncing todos across machines.
[db]
url   = "libsql://your-db-name.turso.io"   # libSQL URL from: turso db show orivo --url
token = "your-auth-token"                  # auth token from: turso db tokens create orivo
# Get your Turso credentials:
#   turso auth login
#   turso db create orivo
#   turso db show orivo --url
#   turso db tokens create orivo


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

### Database (`[db]`)

Orivo uses [Turso](https://turso.tech) as its database — a libSQL-compatible SQLite database. You need a `url` and `token` to connect.

```sh
$ turso auth login
$ turso db create orivo
$ turso db show orivo --url      # → paste as url
$ turso db tokens create orivo   # → paste as token
```

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
