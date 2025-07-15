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
go install github.com/mt-shihab26/termodoro@latest
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
