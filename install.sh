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
EXTRACTED_FILE="${FILE%.tar.gz}"
chmod +x "$EXTRACTED_FILE"
mv "$EXTRACTED_FILE" "$BINARY_NAME"
rm "$FILE"

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

# Verify installation
echo ""
echo "Verifying installation..."

if command -v forg >/dev/null 2>&1; then
  echo "Installation successful!"
  echo "Run: forg --help"
else
  echo "NOT FOUND :::: 'forg' is not in your PATH"

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
