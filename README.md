# orivo

A terminal-based Todos + Pomodoro timer written in Rust

## Configuration

Config file location: `~/.config/orivo/config.toml`

```toml
# Orivo configuration


# Get your Turso credentials:
#   turso auth login
#   turso db create orivo
#   turso db show orivo --url
#   turso db tokens create orivo
[db]
url   = "libsql://your-db-name.turso.io"   # libSQL URL from: turso db show orivo --url
token = "your-auth-token"                  # auth token from: turso db tokens create orivo


[timer]
show_millis         = true   # show milliseconds in the timer display
work_duration       = 25     # work session length in minutes
break_duration      = 5      # short break length in minutes
long_break_duration = 15     # long break length in minutes
long_break_interval = 4      # number of work sessions before a long break
```

### Database (`[db]`)

Orivo uses [Turso](https://turso.tech) as its database — a libSQL-compatible SQLite database built on top of SQLite. You need a `url` and `token` to connect.

```sh
turso auth login
turso db create orivo
turso db show orivo --url      # → paste as url
turso db tokens create orivo   # → paste as token
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
