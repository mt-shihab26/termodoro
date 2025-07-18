#!/bin/env bash

DESKTOP_FILE="$HOME/.local/share/applications/termodoro.desktop"
ICON_FILE="$HOME/.local/share/icons/hicolor/48x48/apps/termodoro.png"

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
    remove_if_exists "$DESKTOP_FILE"
    ensure_directory_exists "$DESKTOP_FILE"

    cp ./termodoro.desktop "$DESKTOP_FILE"
    echo "Desktop file installed successfully!"
}

install_icon_file() {
    remove_if_exists "$ICON_FILE"
    ensure_directory_exists "$ICON_FILE"

    cp ./termodoro.png "$ICON_FILE"
    echo "Icon file installed successfully!"
}

install_desktop_file
install_icon_file
