#!/bin/bash

set -e

# GitHub repository details
REPO_OWNER="mt-shihab26"
REPO_NAME="termodoro"
GITHUB_API_URL="https://api.github.com/repos/${REPO_OWNER}/${REPO_NAME}"

# Installation directory
INSTALL_DIR="${HOME}/.local/bin"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Function to print colored output
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to detect OS
detect_os() {
    case "$(uname -s)" in
    Linux*) echo "linux" ;;
    Darwin*) echo "darwin" ;;
    CYGWIN* | MINGW* | MSYS*) echo "windows" ;;
    *) echo "unknown" ;;
    esac
}

# Function to detect architecture
detect_arch() {
    case "$(uname -m)" in
    x86_64 | amd64) echo "x86_64" ;;
    arm64 | aarch64) echo "arm64" ;;
    *) echo "unknown" ;;
    esac
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to get latest release version
get_latest_version() {
    print_info "Fetching latest release information..."

    if command_exists curl; then
        curl -s "${GITHUB_API_URL}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/'
    elif command_exists wget; then
        wget -qO- "${GITHUB_API_URL}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/'
    else
        print_error "Neither curl nor wget is available. Please install one of them."
        exit 1
    fi
}

# Function to construct download URL
get_download_url() {
    local version="$1"
    local os="$2"
    local arch="$3"

    local filename="termodoro-${version}-${os}-${arch}"
    if [ "$os" = "windows" ]; then
        filename="${filename}.exe"
    fi

    echo "https://github.com/${REPO_OWNER}/${REPO_NAME}/releases/download/${version}/${filename}"
}

# Function to download file
download_file() {
    local url="$1"
    local output="$2"

    print_info "Downloading from: $url"

    if command_exists curl; then
        curl -L -o "$output" "$url"
    elif command_exists wget; then
        wget -O "$output" "$url"
    else
        print_error "Neither curl nor wget is available. Please install one of them."
        exit 1
    fi
}

# Function to verify download
verify_download() {
    local file="$1"

    if [ ! -f "$file" ]; then
        print_error "Downloaded file does not exist: $file"
        exit 1
    fi

    if [ ! -s "$file" ]; then
        print_error "Downloaded file is empty: $file"
        exit 1
    fi

    # Check if it's an HTML error page (common when download fails)
    if file "$file" 2>/dev/null | grep -q "HTML"; then
        print_error "Downloaded file appears to be an HTML page (download may have failed)"
        exit 1
    fi
}

# Function to install binary
install_binary() {
    local downloaded_file="$1"
    local install_path="$2"

    # Create install directory if it doesn't exist
    mkdir -p "$(dirname "$install_path")"

    # Move binary to install location
    mv "$downloaded_file" "$install_path"

    # Make it executable
    chmod +x "$install_path"

    print_success "Installed termodoro to: $install_path"
}

# Function to update PATH
update_path() {
    local install_dir="$1"

    # Check if install directory is in PATH
    if [[ ":$PATH:" != *":$install_dir:"* ]]; then
        print_warning "Install directory ($install_dir) is not in your PATH."
        print_info "Add the following line to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
        echo ""
        echo "    export PATH=\"$install_dir:\$PATH\""
        echo ""
        print_info "Or run the following command to add it to your current session:"
        echo ""
        echo "    export PATH=\"$install_dir:\$PATH\""
        echo ""
    fi
}

# Main installation function
main() {
    print_info "Starting termodoro installation..."

    # Detect system information
    OS=$(detect_os)
    ARCH=$(detect_arch)

    print_info "Detected OS: $OS"
    print_info "Detected Architecture: $ARCH"

    # Validate OS and architecture
    if [ "$OS" = "unknown" ]; then
        print_error "Unsupported operating system: $(uname -s)"
        print_error "Supported: Linux, macOS, Windows"
        exit 1
    fi

    if [ "$ARCH" = "unknown" ]; then
        print_error "Unsupported architecture: $(uname -m)"
        print_error "Supported: x86_64, arm64"
        exit 1
    fi

    # Get latest version
    VERSION=$(get_latest_version)
    if [ -z "$VERSION" ]; then
        print_error "Failed to fetch latest version information"
        exit 1
    fi

    print_info "Latest version: $VERSION"

    # Construct download URL
    DOWNLOAD_URL=$(get_download_url "$VERSION" "$OS" "$ARCH")

    # Create temporary file for download
    TEMP_FILE=$(mktemp)
    trap "rm -f $TEMP_FILE" EXIT

    # Download binary
    if ! download_file "$DOWNLOAD_URL" "$TEMP_FILE"; then
        print_error "Failed to download termodoro"
        exit 1
    fi

    # Verify download
    verify_download "$TEMP_FILE"

    # Determine final binary name
    BINARY_NAME="termodoro"
    if [ "$OS" = "windows" ]; then
        BINARY_NAME="termodoro.exe"
    fi

    INSTALL_PATH="${INSTALL_DIR}/${BINARY_NAME}"

    # Install binary
    install_binary "$TEMP_FILE" "$INSTALL_PATH"

    # Check PATH
    update_path "$INSTALL_DIR"

    # Test installation
    print_info "Testing installation..."
    if "$INSTALL_PATH" --version >/dev/null 2>&1 || "$INSTALL_PATH" -h >/dev/null 2>&1; then
        print_success "Installation completed successfully!"
        print_info "You can now run: termodoro"
    else
        print_warning "Installation completed, but binary test failed."
        print_info "Try running: $INSTALL_PATH"
    fi
}

# Handle command line arguments
case "${1:-}" in
-h | --help)
    echo "Termodoro Installation Script"
    echo ""
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "OPTIONS:"
    echo "  -h, --help     Show this help message"
    echo "  -v, --version  Show version information"
    echo ""
    echo "This script automatically detects your OS and architecture"
    echo "and downloads the appropriate binary from GitHub releases."
    echo ""
    echo "Install directory: $INSTALL_DIR"
    exit 0
    ;;
-v | --version)
    echo "Termodoro Installation Script v1.0.0"
    exit 0
    ;;
"")
    # No arguments, proceed with installation
    main
    ;;
*)
    print_error "Unknown option: $1"
    print_info "Use -h or --help for usage information"
    exit 1
    ;;
esac
