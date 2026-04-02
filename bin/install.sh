#!/usr/bin/env bash
set -euo pipefail

REPO="mt-shihab26/orivo"
BINARY="orivo"
PREFIX="${HOME}/.local"
TERMINAL=""

# ---- Config (derived after args are parsed) ----
BIN_DIR=""
ICON_DIR=""
APPS_DIR=""
OS_TAG=""
ARCH_TAG=""
FETCH=""
VERSION=""
TMP=""

# ------------------------------------------------------------------ #
#  Argument parsing                                                    #
# ------------------------------------------------------------------ #
parse_args() {
    while [[ $# -gt 0 ]]; do
        case "$1" in
        --prefix)
            PREFIX="$2"
            shift 2
            ;;
        --prefix=*)
            PREFIX="${1#--prefix=}"
            shift
            ;;
        --terminal)
            TERMINAL="$2"
            shift 2
            ;;
        --terminal=*)
            TERMINAL="${1#--terminal=}"
            shift
            ;;
        -h | --help)
            echo "Usage: $0 [--prefix DIR] [--terminal kitty|alacritty]"
            echo "  --prefix DIR                Install to DIR/bin (default: ~/.local)"
            echo "  --terminal kitty|alacritty  Terminal for the desktop entry"
            exit 0
            ;;
        *)
            echo "Unknown option: $1" >&2
            exit 1
            ;;
        esac
    done

    BIN_DIR="$PREFIX/bin"
    ICON_DIR="$PREFIX/share/icons/hicolor/scalable/apps"
    APPS_DIR="$PREFIX/share/applications"
}

# ------------------------------------------------------------------ #
#  Detect OS and architecture                                          #
# ------------------------------------------------------------------ #
detect_platform() {
    local os arch
    os=$(uname -s)
    arch=$(uname -m)
    case "$os" in Linux) OS_TAG=linux ;; Darwin) OS_TAG=macos ;; *)
        echo "Unsupported OS: $os" >&2
        exit 1
        ;;
    esac
    case "$arch" in x86_64) ARCH_TAG=x86_64 ;; aarch64 | arm64) ARCH_TAG=aarch64 ;; *)
        echo "Unsupported arch: $arch" >&2
        exit 1
        ;;
    esac
}

# ------------------------------------------------------------------ #
#  Detect download tool                                                #
# ------------------------------------------------------------------ #
detect_fetch() {
    if command -v curl &>/dev/null; then
        FETCH="curl -fsSL"
    elif command -v wget &>/dev/null; then
        FETCH="wget -qO-"
    else
        echo "ERROR: curl or wget is required." >&2
        exit 1
    fi
}

# ------------------------------------------------------------------ #
#  Install sqlite3 system dependency                                   #
# ------------------------------------------------------------------ #
install_sqlite() {
    echo "Checking sqlite3..."
    command -v sqlite3 &>/dev/null && {
        echo "sqlite3 already installed."
        return
    }
    echo "sqlite3 not found — installing..."
    if command -v pacman &>/dev/null; then
        sudo pacman -S --noconfirm sqlite
    elif command -v apt-get &>/dev/null; then
        sudo apt-get install -y libsqlite3-dev
    elif command -v dnf &>/dev/null; then
        sudo dnf install -y sqlite-devel
    elif command -v zypper &>/dev/null; then
        sudo zypper install -y sqlite3-devel
    elif command -v apk &>/dev/null; then
        sudo apk add --no-cache sqlite-dev
    elif command -v brew &>/dev/null; then
        brew install sqlite
    elif command -v port &>/dev/null; then
        sudo port install sqlite3
    else
        echo "WARNING: Unknown package manager. Install sqlite3 manually." >&2
    fi
}

# ------------------------------------------------------------------ #
#  Resolve latest release version from GitHub                         #
# ------------------------------------------------------------------ #
resolve_version() {
    VERSION=$(${FETCH} "https://api.github.com/repos/${REPO}/releases/latest" |
        grep '"tag_name"' | sed 's/.*"\([^"]*\)".*/\1/')
    [ -z "$VERSION" ] && {
        echo "ERROR: Could not fetch latest version." >&2
        exit 1
    }
}

# ------------------------------------------------------------------ #
#  Download and install binary                                         #
# ------------------------------------------------------------------ #
install_binary() {
    local archive="${BINARY}-${VERSION}-${OS_TAG}-${ARCH_TAG}.tar.gz"
    echo "Installing ${BINARY} ${VERSION} (${OS_TAG}/${ARCH_TAG}) -> ${BIN_DIR}..."

    TMP=$(mktemp -d)
    trap 'rm -rf "$TMP"' EXIT
    ${FETCH} "https://github.com/${REPO}/releases/download/${VERSION}/${archive}" >"$TMP/$archive"
    tar xzf "$TMP/$archive" -C "$TMP"

    mkdir -p "$BIN_DIR"
    install -m 755 "$TMP/$BINARY" "$BIN_DIR/$BINARY"
    echo "Binary:  $BIN_DIR/$BINARY"
}

# ------------------------------------------------------------------ #
#  Prompt for terminal selection (Linux only)                          #
# ------------------------------------------------------------------ #
pick_terminal() {
    [ -n "$TERMINAL" ] && return
    printf "Select terminal for the desktop entry:\n"
    printf "  1) kitty      (default)\n"
    printf "  2) alacritty\n"
    printf "Choice [1]: "
    read -r choice
    case "${choice:-1}" in
    2 | alacritty) TERMINAL=alacritty ;;
    *) TERMINAL=kitty ;;
    esac
}

# ------------------------------------------------------------------ #
#  Install desktop entry and icon (Linux only)                         #
# ------------------------------------------------------------------ #
install_desktop() {
    [ "$OS_TAG" != linux ] && return

    pick_terminal

    local desktop_src
    case "$TERMINAL" in
    kitty) desktop_src="orivo-kitty.desktop" ;;
    alacritty) desktop_src="orivo-alacritty.desktop" ;;
    *)
        echo "ERROR: Unknown terminal '$TERMINAL'. Choose kitty or alacritty." >&2
        exit 1
        ;;
    esac

    # Fetch desktop file and icon directly from the repo (works with curl | bash)
    mkdir -p "$APPS_DIR" "$ICON_DIR"
    local base="https://raw.githubusercontent.com/${REPO}/main/xdg"
    ${FETCH} "${base}/${desktop_src}" >"$APPS_DIR/orivo.desktop"
    ${FETCH} "${base}/orivo.svg" >"$ICON_DIR/orivo.svg"

    command -v update-desktop-database &>/dev/null && update-desktop-database "$APPS_DIR" 2>/dev/null || true
    echo "Terminal: $TERMINAL"
    echo "Desktop:  $APPS_DIR/orivo.desktop"
    echo "Icon:     $ICON_DIR/orivo.svg"
}

# ------------------------------------------------------------------ #
#  PATH reminder                                                       #
# ------------------------------------------------------------------ #
check_path() {
    echo "$PATH" | tr ':' '\n' | grep -qx "$BIN_DIR" && return
    printf "\nNOTE: Add to your shell profile:\n  export PATH=\"\$PATH:%s\"\n" "$BIN_DIR"
}

# ------------------------------------------------------------------ #
#  Main                                                                #
# ------------------------------------------------------------------ #
main() {
    parse_args "$@"
    detect_platform
    detect_fetch
    install_sqlite
    resolve_version
    install_binary
    install_desktop
    check_path
    printf "\nRun 'orivo' to start. Config: ~/.config/orivo/config.toml\n"
}

main "$@"
