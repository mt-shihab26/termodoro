<div>
  <img src="./assets/logo.svg" alt="Termodoro Logo" width="96" height="96">
</div>

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

### Manual Installation

#### From Source

```bash
go install github.com/mt-shihab26/termodoro/cmd/termodoro@latest
```

#### Download Binary

You can manually download pre-built binaries from the [releases page](https://github.com/mt-shihab26/termodoro/releases/latest).

Available for:

- Linux (x86_64, ARM64)
- macOS (x86_64, ARM64)
- Windows (x86_64, ARM64)

## Usage

```bash
termodoro
```

### Controls

- `SPACE`: Start/Pause timer
- `R`: Reset current session
- `B`: Start break manually
- `Q`: Quit application

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
    "work_session_duration": 25,
    "break_session_duration": 5,
    "long_break_session_duration": 15
}
```

### Configuration Options

| Option                        | Description             | Default | Unit    |
| ----------------------------- | ----------------------- | ------- | ------- |
| `work_session_duration`       | Length of work sessions | 25      | minutes |
| `break_session_duration`      | Length of short breaks  | 5       | minutes |
| `long_break_session_duration` | Length of long breaks   | 15      | minutes |

### Partial Configuration

You can specify only the options you want to customize. Missing options will use their default values:

```json
{
    "work_session_duration": 30
}
```

In this example, work sessions will be 30 minutes, while breaks will use the default values (5 and 15 minutes).

### Configuration Examples

**Extended Work Sessions:**

```json
{
    "work_session_duration": 45,
    "break_session_duration": 10,
    "long_break_session_duration": 30
}
```

**Short Sessions for ADHD/Focus Issues:**

```json
{
    "work_session_duration": 15,
    "break_session_duration": 5,
    "long_break_session_duration": 15
}
```

**Minimal Breaks:**

```json
{
    "work_session_duration": 25,
    "break_session_duration": 2,
    "long_break_session_duration": 10
}
```

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

This will create a binary in the root `./termodoro`

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.

## Author

**Shihab Mahamud**

- Website: [developershihab.com](https://developershihab.com)
