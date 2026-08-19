#!/usr/bin/env bash

set -e

INSTALL_DIR="$HOME/.local/bin"
CFX_PATH="$INSTALL_DIR/cfx"

if [ -f "$CFX_PATH" ]; then
    rm "$CFX_PATH"
    echo "CFX binary successfully removed from $INSTALL_DIR."
else
    echo "CFX binary not found at $CFX_PATH. It might already be uninstalled."
fi

# Optional cleanup: if we had a config folder, remove it
if [ -d "$HOME/.cfx" ]; then
    rm -rf "$HOME/.cfx"
    echo "Removed CFX configuration directory."
fi

echo "CFX has been uninstalled."
echo "Note: If CFX was added to your PATH in ~/.bashrc or ~/.zshrc, you may remove that line manually."
