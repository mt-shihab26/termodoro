# orivo

A terminal-based Todos + Pomodoro timer written in Rust

## Configuration

Config file location: `~/.config/orivo/config.toml`

```toml
# Orivo configuration
#
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

