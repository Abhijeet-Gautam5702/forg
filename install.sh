#!/bin/bash
set -e

REPO="Abhijeet-Gautam5702/forg"
BINARY_NAME="forg"

# Detect architecture of the system
ARCH=$(uname -m)

if [[ "$ARCH" == "arm64" ]]; then
  FILE="forg-macos-arm64.tar.gz"
elif [[ "$ARCH" == "x86_64" ]]; then
  FILE="forg-macos-x86_64.tar.gz"
else
  echo "Unsupported architecture: $ARCH"
  exit 1
fi

# Latest release URL
URL="https://github.com/$REPO/releases/latest/download/$FILE"

echo "Downloading $FILE..."

curl -L "$URL" -o "$FILE"

echo "Extracting..."
tar -xzf "$FILE"

chmod +x "$BINARY_NAME"

# Install location
INSTALL_DIR="/usr/local/bin"

if [[ ! -w "$INSTALL_DIR" ]]; then
  echo "Installing to ~/.local/bin instead..."
  INSTALL_DIR="$HOME/.local/bin"
  mkdir -p "$INSTALL_DIR"
fi

mv "$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"

echo "Installed to $INSTALL_DIR/$BINARY_NAME"

echo "Done! Run: $BINARY_NAME --help"
