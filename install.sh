#!/bin/bash
set -e

# Constants
REPO="Abhijeet-Gautam5702/forg"
BINARY_NAME="forg"

# Detect architecture of the system & decide the tar.gz file to download
ARCH=$(uname -m)
OS=$(uname -s)

if [[ "$OS" == "Darwin" ]]; then
    if [[ "$ARCH" == "arm64" ]]; then
        FILE="forg-macos-arm64.tar.gz"
    else
        FILE="forg-macos-x86_64.tar.gz"
    fi
elif [[ "$OS" == "Linux" ]]; then
    if [[ "$ARCH" == "x86_64" ]]; then
        FILE="forg-linux-x86_64.tar.gz"
    else
        echo "[ERROR] Unsupported Linux Architecture: $ARCH"
        exit 1
    fi
else
    echo "[ERROR] Unsupported OS: $OS"
    exit 1
fi

# Get the latest binary present on github releases
LATEST_VERSION=$(curl -s "https://api.github.com/repos/Abhijeet-Gautam5702/forg/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/' | sed 's/^v//')
echo "Installing forg v$LATEST_VERSION"
# Release URL (LATEST VERSION)
URL="https://github.com/$REPO/releases/latest/download/$FILE"

echo "Fetching from $URL"
echo "Downloading $FILE..."

curl -L "$URL" -o "$FILE" || {
 echo "[ERROR] Download failed"
 exit 1
}

echo "Extracting $FILE ...."
tar -xzf "$FILE" || {
 echo "[ERROR] Extraction failed"
 exit 1
}
EXTRACTED_FILE="${FILE%.tar.gz}"
chmod +x "$EXTRACTED_FILE"
mv "$EXTRACTED_FILE" "$BINARY_NAME"
echo "Cleanup: Removing $FILE"
rm "$FILE"

# Install location
INSTALL_DIR="/usr/local/bin"

# Write permission denied to /usr/local/bin -> Install to local bin of the user
if [[ ! -w "$INSTALL_DIR" ]]; then
  echo "[WARN] No write access to /usr/local/bin"
  echo "Installing to ~/.local/bin instead..."
  echo "To install globally, re-run with 'sudo'"
  INSTALL_DIR="$HOME/.local/bin"
  mkdir -p "$INSTALL_DIR"
fi

# Binary already exists in install location -> Overwrite
if [[ -f "$INSTALL_DIR/$BINARY_NAME" ]]; then
  echo "Existing installation found at $INSTALL_DIR/$BINARY_NAME"
  echo "Overwriting the existing binary..."
fi
mv "$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"

echo "Installed to $INSTALL_DIR/$BINARY_NAME"

# Verify installation
echo ""
echo "Verifying installation..."

if command -v forg >/dev/null 2>&1; then
  echo "forg v$LATEST_VERSION Installation successful!"
  echo "Run: forg --help"
else
  echo "[ERROR] 'forg' is not in your PATH"

  # Detect shell
  SHELL_NAME=$(basename "$SHELL")

  echo ""
  echo "👉 To fix this, add ~/.local/bin to your PATH"

  if [[ "$SHELL_NAME" == "zsh" ]]; then
    echo ""
    echo "For zsh (default on macOS):"
    echo "  nano ~/.zshrc"
    echo ""
    echo "Add this line:"
    echo '  export PATH="$HOME/.local/bin:$PATH"'
    echo ""
    echo "Then run:"
    echo "  source ~/.zshrc"

  elif [[ "$SHELL_NAME" == "bash" ]]; then
    echo ""
    echo "For bash:"
    echo "  nano ~/.bashrc"
    echo ""
    echo "Add this line:"
    echo '  export PATH="$HOME/.local/bin:$PATH"'
    echo ""
    echo "Then run:"
    echo "  source ~/.bashrc"

  else
    echo ""
    echo "Unknown shell: $SHELL_NAME"
    echo "Add this line to your shell config file:"
    echo '  export PATH="$HOME/.local/bin:$PATH"'
  fi

  echo ""
  echo "After updating PATH, restart your terminal or source your config file."
fi
