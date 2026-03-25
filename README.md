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

## Installation

Requires [Rust](https://www.rust-lang.org/tools/install).

```bash
git clone https://github.com/yourusername/termodoro
cd termodoro
cargo build --release
```

The binary will be at `./target/release/termodoro`. You can move it to a directory on your `$PATH`:

```bash
cp target/release/termodoro ~/.local/bin/
```

## Usage

```bash
termodoro
```

### Controls

| Key        | Action                   |
|------------|--------------------------|
| `Space`    | Pause / Resume           |
| `s`        | Skip to next phase       |
| `r`        | Reset current phase      |
| `q` / `Ctrl+C` | Quit               |

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
