#!/usr/bin/env bash

set -e

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

if [ "$OS" = "Linux" ]; then
    ASSET_NAME="cfx-linux-x86_64"
elif [ "$OS" = "Darwin" ]; then
    ASSET_NAME="cfx-macos-x86_64"
else
    echo "Unsupported OS: $OS"
    exit 1
fi

if [ "$ARCH" != "x86_64" ] && [ "$ARCH" != "amd64" ] && [ "$ARCH" != "arm64" ]; then
    echo "Warning: Only x86_64 binaries are currently pre-compiled. You might need to build from source."
fi

echo "Fetching latest CFX release..."
# Get latest release tag from GitHub API
LATEST_TAG=$(curl -s "https://api.github.com/repos/raushan728/cfx/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_TAG" ]; then
    # Fallback to fetching tags directly if no 'latest' is flagged
    LATEST_TAG=$(curl -s "https://api.github.com/repos/raushan728/cfx/tags" | grep '"name":' | head -n 1 | sed -E 's/.*"([^"]+)".*/\1/')
    if [ -z "$LATEST_TAG" ]; then
        echo "Error: Could not find any releases."
        exit 1
    fi
fi

DOWNLOAD_URL="https://github.com/raushan728/cfx/releases/download/${LATEST_TAG}/${ASSET_NAME}"

# Define installation directory
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

echo "Downloading $DOWNLOAD_URL ..."
TMP_DIR=$(mktemp -d)
curl -L -o "$TMP_DIR/$ASSET_NAME" "$DOWNLOAD_URL"

# Move the binary
mv "$TMP_DIR/$ASSET_NAME" "$INSTALL_DIR/cfx"
chmod +x "$INSTALL_DIR/cfx"

rm -rf "$TMP_DIR"

# Add to PATH if not already there
SHELL_NAME=$(basename "$SHELL")
PROFILE_FILE=""

if [ "$SHELL_NAME" = "bash" ]; then
    if [ -f "$HOME/.bash_profile" ]; then
        PROFILE_FILE="$HOME/.bash_profile"
    else
        PROFILE_FILE="$HOME/.bashrc"
    fi
elif [ "$SHELL_NAME" = "zsh" ]; then
    PROFILE_FILE="$HOME/.zshrc"
fi

if [ -n "$PROFILE_FILE" ]; then
    if ! grep -q "$INSTALL_DIR" "$PROFILE_FILE"; then
        echo -e "\nexport PATH=\"$INSTALL_DIR:\$PATH\"" >> "$PROFILE_FILE"
        echo "Added $INSTALL_DIR to $PROFILE_FILE."
        echo "Please restart your terminal or run: source $PROFILE_FILE"
    fi
else
    echo "Warning: Could not automatically add $INSTALL_DIR to your PATH."
    echo "Please add it manually to your shell configuration."
fi

echo "CFX successfully installed to $INSTALL_DIR/cfx!"
echo "Run 'cfx --help' to get started."
