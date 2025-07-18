#!/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

DESKTOP_FILE_SOURCE="$SCRIPT_DIR/termodoro.desktop"
DESKTOP_FILE_DESTINATION="$HOME/.local/share/applications/termodoro.desktop"

ICON_FILE_SOURCE="$PROJECT_ROOT/assets/logo.png"
ICON_FILE_DESTINATION="$HOME/.local/share/icons/hicolor/48x48/apps/termodoro.png"

ensure_directory_exists() {
    local dir="$(dirname "$1")"
    if [ ! -d "$dir" ]; then
        mkdir -p "$dir"
    fi
}

remove_if_exists() {
    if [ -f "$1" ]; then
        rm "$1"
    fi
}

install_desktop_file() {
    remove_if_exists "$DESKTOP_FILE_DESTINATION"
    ensure_directory_exists "$DESKTOP_FILE_DESTINATION"

    cp "$DESKTOP_FILE_SOURCE" "$DESKTOP_FILE_DESTINATION"
    echo "Desktop file installed successfully!"
}

install_icon_file() {
    remove_if_exists "$ICON_FILE_DESTINATION"
    ensure_directory_exists "$ICON_FILE_DESTINATION"

    cp "$ICON_FILE_SOURCE" "$ICON_FILE_DESTINATION"
    echo "Icon file installed successfully!"
}

install_desktop_file
install_icon_file
