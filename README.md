<div>
  <img src="./assets/logo.svg" alt="Termodoro Logo" width="96" height="96">
</div>

# Termodoro

A terminal-based Pomodoro timer application built with Go.

## Features

- A TUI application
- Configurable work sessions (default: 25 minutes)
- Configurable short breaks (default: 5 minutes)
- Configurable long breaks (default: 15 minutes, every 4th cycle)
- Keyboard controls

## Installation

### Install Binary (Recommended)

Install the latest version automatically for your operating system:

```bash
curl -fsSL "https://raw.githubusercontent.com/mt-shihab26/termodoro/main/install.sh?$(date +%s)" | bash
```

The installer will:

- Automatically detect your OS and architecture
- Download the latest release binary
- Install to `~/.local/bin/termodoro`
- Set executable permissions
- Provide PATH setup instructions if needed

### Download Binary

You can manually download pre-built binaries from the [releases page](https://github.com/mt-shihab26/termodoro/releases/latest).

Available for:

- Linux (x86_64, ARM64)
- macOS (x86_64, ARM64)
- Windows (x86_64, ARM64)

### From Source

```bash
go install github.com/mt-shihab26/termodoro/cmd/termodoro@latest
```

## Usage

### Commands

Start the Pomodoro timer application:

```bash
termodoro
```

Display version information:

```bash
termodoro version
```

Show help and usage information:

```bash
termodoro help
```

### Controls

- `space`: Start/Pause timer
- `r`: Reset current session
- `n`: Go to next session manually
- `q`: Quit application

## Configuration

Termodoro supports user configuration through a JSON file located at `~/.config/termodoro/config.json`. If no config file exists, the application will use sensible defaults based on the standard Pomodoro Technique.

### Config File Location

The configuration file follows the XDG Base Directory Specification:

```
~/.config/termodoro/config.json
```

### Config File Format

Create a JSON file with any or all of the following options:

```json
{
    "work_session_duration": 55,
    "long_break_session_interval": 2
}
```

### Configuration Options

| Option                        | Description                                 | Default | Unit          |
| ----------------------------- | ------------------------------------------- | ------- | ------------- |
| `work_session_duration`       | Length of work sessions                     | 25      | minutes       |
| `break_session_duration`      | Length of short breaks                      | 5       | minutes       |
| `long_break_session_duration` | Length of long breaks                       | 15      | minutes       |
| `long_break_session_interval` | Number of work sessions before a long break | 4       | work sessions |

## Development

### Prerequisites

- [Go](https://golang.org/dl/) 1.24 or newer
- [Make](https://www.gnu.org/software/make/)

### Run Locally

To build and run the app using the provided `Makefile`:

```bash
make run
```

To build only:

```bash
make build
```

This will create a binary in the root directory as `./termodoro`.

**Note:** The install script places the binary at `~/.local/bin/termodoro`, which is where the automated installation puts the executable.
