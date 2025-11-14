#!/bin/bash
# Lala Editor launcher with IME support
# This script ensures IME (Input Method Editor) works correctly on Linux

# Detect which IME is running
if pgrep -x "ibus-daemon" > /dev/null; then
    echo "Detected ibus, setting environment variables..."
    export GTK_IM_MODULE=ibus
    export XMODIFIERS=@im=ibus
    export QT_IM_MODULE=ibus
elif pgrep -x "fcitx" > /dev/null || pgrep -x "fcitx5" > /dev/null; then
    echo "Detected fcitx, setting environment variables..."
    export GTK_IM_MODULE=fcitx
    export XMODIFIERS=@im=fcitx
    export QT_IM_MODULE=fcitx
else
    echo "No IME detected, using default settings"
fi

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Check if lala binary exists
if [ -f "$SCRIPT_DIR/target/release/lala" ]; then
    echo "Starting Lala Editor..."
    exec "$SCRIPT_DIR/target/release/lala" "$@"
elif [ -f "$SCRIPT_DIR/lala" ]; then
    echo "Starting Lala Editor..."
    exec "$SCRIPT_DIR/lala" "$@"
else
    echo "Error: lala binary not found"
    echo "Please build the project first with: cargo build --release"
    exit 1
fi
