# Installation

Getting **forg** onto your system is quick and easy. We provide a one-line installer for most Unix-like systems.

## One-Line Install

Run the following command in your terminal to download and install the latest version of **forg**:

```bash
curl -sSL https://raw.githubusercontent.com/Abhijeet-Gautam5702/forg/main/install.sh | bash
```

This script will:
1. Detect your Operating System and Architecture.
2. Download the appropriate binary.
3. Place it in `~/.local/bin` (or `/usr/local/bin` if root).

## Verifying Installation

After installation, verify that **forg** is correctly installed and accessible:

```bash
which forg
# output: "/Users/<username>/.local/bin/forg" OR "/usr/local/bin/forg"

forg --version
```

If you receive a `command not found` error, you may need to add `~/.local/bin` to your `PATH`.

### Adding to PATH (macOS/Linux)

**For Zsh (default on macOS):**
```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

**For Bash:**
```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## Build from Source

Ensure you have [Rust and Cargo](https://rustup.rs/) installed, then clone and build:

```bash
git clone https://github.com/Abhijeet-Gautam5702/forg.git
cd forg
cargo build --release
```

After building, the binary will be located at `./target/release/forg`. You can move it to your path manually:
```bash
mv ./target/release/forg ~/.local/bin/
```

## Updating

**forg** can update itself! Simply run:

```bash
forg self-update
```

Alternatively, you can re-run the installation script to overwrite the existing binary with the latest version.

## Uninstallation

If you wish to remove **forg** and its configuration files:

```bash
forg uninstall
```

{DANGER type="admonition" title="Manual Cleanup"}
If the `uninstall` command is not available (versions < 0.1.4), you can manually remove the binary and config:
```bash
rm $(which forg)
rm -rf ~/.forg
```
{/DANGER}
