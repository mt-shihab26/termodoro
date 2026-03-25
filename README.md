# termodoro

A terminal-based Pomodoro timer built in Rust. Helps you manage focused work sessions and breaks using the Pomodoro Technique.

## Features

- Three-phase timer: Work, Short Break, and Long Break
- Pause, resume, skip, and reset controls
- Progress bar and session counter
- Color-coded phases for quick visual reference
- Persistent state — resumes where you left off
- Configurable durations
- Follows XDG Base Directory spec

## Usage

```bash
termodoro
```

## Configuration

Create a config file at `~/.config/termodoro/config.json` (or `$XDG_CONFIG_HOME/termodoro/config.json`):

```json
{
  "work_session_duration": 25,
  "break_session_duration": 5,
  "long_break_session_duration": 15,
  "long_break_session_interval": 4
}
```

All values are in minutes. The application uses defaults if the file is missing or invalid.

| Option | Default | Description |
|--------|---------|-------------|
| `work_session_duration` | `25` | Length of a work session |
| `break_session_duration` | `5` | Length of a short break |
| `long_break_session_duration` | `15` | Length of a long break |
| `long_break_session_interval` | `4` | Work sessions before a long break |

## State & Logs

- **State**: `~/.local/state/termodoro/state.json` — saves current phase, time remaining, and session count
- **Logs**: `~/.local/state/termodoro/termodoro.log` — records errors during config/state loading

