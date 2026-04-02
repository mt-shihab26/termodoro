#!/usr/bin/env bash
set -euo pipefail

detect_platform() {
    local os arch os_tag arch_tag
    os=$(uname -s)
    arch=$(uname -m)
    case "$os" in
    Linux) os_tag="linux" ;;
    Darwin) os_tag="macos" ;;
    *)
        echo "Unsupported OS: $os" >&2
        exit 1
        ;;
    esac
    case "$arch" in
    x86_64) arch_tag="x86_64" ;;
    aarch64 | arm64) arch_tag="aarch64" ;;
    *)
        echo "Unsupported arch: $arch" >&2
        exit 1
        ;;
    esac
    printf "%s %s\n" "$os_tag" "$arch_tag"
}

detect_fetch() {
    if command -v curl &>/dev/null; then
        echo "curl -fsSL"
    elif command -v wget &>/dev/null; then
        echo "wget -qO-"
    else
        echo "ERROR: curl or wget is required." >&2
        exit 1
    fi
}

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

resolve_current_version() {
    local bin_dir="$1"
    local bin="${bin_dir}/orivo"
    [ -x "$bin" ] && "$bin" --version 2>/dev/null || echo ""
}

resolve_version() {
    local fetch="$1" repo="$2"
    local version
    version=$(${fetch} "https://api.github.com/repos/${repo}/releases/latest" |
        grep '"tag_name"' | sed 's/.*"\([^"]*\)".*/\1/')
    [ -z "$version" ] && {
        echo "ERROR: Could not fetch latest version." >&2
        exit 1
    }
    echo "$version"
}

verify_checksum() {
    local file="$1" archive="$2" checksums="$3"
    echo "Verifying checksum..."
    local expected
    expected=$(grep "${archive}" "$checksums" | awk '{print $1}')
    [ -z "$expected" ] && {
        echo "ERROR: No checksum found for ${archive}" >&2
        exit 1
    }

    local actual
    if command -v sha256sum &>/dev/null; then
        actual=$(sha256sum "$file" | awk '{print $1}')
    elif command -v shasum &>/dev/null; then
        actual=$(shasum -a 256 "$file" | awk '{print $1}')
    else
        echo "WARNING: No sha256 tool found, skipping verification." >&2
        return
    fi

    [ "$actual" = "$expected" ] || {
        echo "ERROR: Checksum mismatch for ${archive}" >&2
        exit 1
    }
    echo "Checksum OK."
}

install_binary() {
    local fetch="$1" repo="$2" binary="$3" version="$4" os_tag="$5" arch_tag="$6" bin_dir="$7"
    local archive="${binary}-${version}-${os_tag}-${arch_tag}.tar.gz"
    local base="https://github.com/${repo}/releases/download/${version}"
    echo "Installing ${binary} ${version} (${os_tag}/${arch_tag}) -> ${bin_dir}..."

    local tmp
    tmp=$(mktemp -d)
    trap "rm -rf -- '$tmp'" EXIT
    ${fetch} "${base}/${archive}" >"$tmp/$archive"
    ${fetch} "${base}/SHA256SUMS.txt" >"$tmp/SHA256SUMS.txt"

    verify_checksum "$tmp/$archive" "$archive" "$tmp/SHA256SUMS.txt"

    tar xzf "$tmp/$archive" -C "$tmp"
    mkdir -p "$bin_dir"
    install -m 755 "$tmp/$binary" "$bin_dir/$binary"
    echo "Binary:  $bin_dir/$binary"
}

pick_terminal() {
    local terminal="$1"
    [ -n "$terminal" ] && {
        echo "$terminal"
        return
    }
    echo "kitty"
}

install_desktop() {
    local fetch="$1" repo="$2" os_tag="$3" terminal="$4" apps_dir="$5" icon_dir="$6"
    [ "$os_tag" != linux ] && return

    terminal=$(pick_terminal "$terminal")

    local desktop_src
    case "$terminal" in
    kitty) desktop_src="orivo-kitty.desktop" ;;
    alacritty) desktop_src="orivo-alacritty.desktop" ;;
    *)
        echo "ERROR: Unknown terminal '$terminal'. Choose kitty or alacritty." >&2
        exit 1
        ;;
    esac

    mkdir -p "$apps_dir" "$icon_dir"
    local base="https://raw.githubusercontent.com/${repo}/main/xdg"
    ${fetch} "${base}/${desktop_src}" >"$apps_dir/orivo.desktop"
    ${fetch} "${base}/orivo.svg" >"$icon_dir/orivo.svg"

    # Patch Icon to absolute path
    sed -i "s|Icon=orivo|Icon=${icon_dir}/orivo.svg|" "$apps_dir/orivo.desktop"

    command -v update-desktop-database &>/dev/null && update-desktop-database "$apps_dir" 2>/dev/null || true
    echo "Terminal: $terminal"
    echo "Desktop:  $apps_dir/orivo.desktop"
    echo "Icon:     $icon_dir/orivo.svg"
}

check_path() {
    local bin_dir="$1"
    echo "$PATH" | tr ':' '\n' | grep -qx "$bin_dir" && return
    printf "\nNOTE: Add to your shell profile:\n  export PATH=\"\$PATH:%s\"\n" "$bin_dir"
}

REPO="mt-shihab26/orivo"
BINARY="orivo"
BIN_DIR="${HOME}/.local/bin"
ICON_DIR="${HOME}/.local/share/icons/hicolor/scalable/apps"
APPS_DIR="${HOME}/.local/share/applications"

TERMINAL=""

while [[ $# -gt 0 ]]; do
    case "$1" in
    --terminal)
        TERMINAL="$2"
        shift 2
        ;;
    --terminal=*)
        TERMINAL="${1#--terminal=}"
        shift
        ;;
    -h | --help)
        echo "Usage: $0 [--terminal kitty|alacritty]"
        echo "  --terminal kitty|alacritty  Terminal for the desktop entry"
        exit 0
        ;;
    *)
        echo "Unknown option: $1" >&2
        exit 1
        ;;
    esac
done

main() {
    local fetch os_tag arch_tag version current_version
    read -r os_tag arch_tag < <(detect_platform)
    fetch=$(detect_fetch)
    install_sqlite
    version=$(resolve_version "$fetch" "$REPO")
    current_version=$(resolve_current_version "$BIN_DIR")

    # Strip leading 'v' from release tag for comparison (tag: v0.1.0, binary: 0.1.0)
    if [ "${version#v}" = "$current_version" ] && [ -n "$current_version" ]; then
        echo "Already up to date (${current_version})."
        exit 0
    fi

    [ -n "$current_version" ] && echo "Upgrading ${current_version} -> ${version#v}..." || true

    install_binary "$fetch" "$REPO" "$BINARY" "$version" "$os_tag" "$arch_tag" "$BIN_DIR"
    install_desktop "$fetch" "$REPO" "$os_tag" "$TERMINAL" "$APPS_DIR" "$ICON_DIR"
    check_path "$BIN_DIR"
    printf "\nRun 'orivo' to start. Config: ~/.config/orivo/config.toml\n"
}

main
