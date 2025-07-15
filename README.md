# Termodoro

A terminal-based Pomodoro timer application built with Go.

## Features

- A TUI Application
- Configurable work sessions (default: 25-minute)
- Configurable short breaks (default: 5-minute)
- Configurable long breaks (default: 15-minute, every 4th cycle)
- Visual progress indicators
- Keyboard controls

## Installation

```bash
go install github.com/mt-shihab26/termodoro/cmd/termodoro@latest
```

## Usage

```bash
termodoro
```

### Controls

- `SPACE`: Start/Pause timer
- `R`: Reset current session
- `B`: Start break manually
- `Q`: Quit application
