# 🦀 forg

[![Built with Rust](https://img.shields.io/badge/Built_with-Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**forg** is a high-performance, regex-powered file organization CLI tool built in Rust. It automates the tedious task of sorting files into designated directories based on custom pattern-matching rules, ensuring your workspace remains clean and structured.

---

## Features

- **Regex-Powered Matching**: Use the full power of Rust's regex engine to define complex file-sorting rules.
- **Priority-Based Sorting**: Rules are processed in the order they appear in your config (the first match wins).
- **Safety First**: 
  - **Dry Run Mode**: Preview exactly what will happen before moving a single byte.
  - **Overwrite Protection**: Never lose data; forg refuses to overwrite existing files in the destination.
  - **Hidden File Guard**: System and hidden files (starting with '.') are ignored by default.
- **Auto-Directory Creation**: Missing destination folders are created on-the-fly only when a match is found.
- **Optimized Execution**: Regex patterns are pre-compiled once to ensure maximum throughput even in directories with thousands of files.

---

## Installation

To install/update **forg** using the installation script, run:

```bash
curl -sSL https://raw.githubusercontent.com/Abhijeet-Gautam5702/forg/main/install.sh | bash
```

### Verify Installation
After running the script, verify that **forg** is correctly installed and accessible:
```bash
which forg
# output: /Users/<username>/.local/bin/forg
```

### Troubleshooting (PATH issues)
If you get a 'command not found' error, ensure `~/.local/bin` is in your `PATH`. Depending on your shell, follow these steps:

**For zsh (default on macOS):**
1. Open shell config: `nano ~/.zshrc`
2. Add this line: `export PATH="$HOME/.local/bin:$PATH"`
3. Reload config: `source ~/.zshrc`

**For bash:**
1. Open shell config: `nano ~/.bashrc`
2. Add this line: `export PATH="$HOME/.local/bin:$PATH"`
3. Reload config: `source ~/.bashrc`

### Supported Platforms
- **macOS**: arm64 (Apple Silicon) & x86_64 (Intel)
- **Linux**: x86_64

---

## Build from Source

Ensure you have [Rust and Cargo](https://rustup.rs/) installed, then clone and build:
```bash
git clone https://github.com/Abhijeet-Gautam5702/forg.git
cd forg
cargo build --release
```

---

## Getting Started

### 1. Initialization
Set up your global configuration file in `~/.forg/config.json`:
```bash
forg --init
```

### 2. Organize
Sort your files (it's recommended to preview first using the dry-run mode):
```bash
# Preview (doesn't move files, but shows the files which will be moved)
forg --exec Downloads --dry-run

# Execute (actually move files)
forg --exec Downloads
```
> **Note:** By default, `DIR_PATH` (here, 'Downloads') directory is relative to the home directory.

---

## Configuration

You can fully customize how **forg** organizes your files by tweaking the configuration file at `~/.forg/config.json`. Rules are matched from **top to bottom**, meaning the first pattern that matches a file determines its destination.

Just define the path (**relative to home**) for each regex pattern, and you're good to go.

### Example config.json
```json
[
  {
    "pattern": ".*Screenshot.*",
    "path": "Desktop/Screenshots"
  },
  {
    "pattern": ".*\\.(png|jpeg|jpg)$",
    "path": "Pictures"
  },
  {
    "pattern": ".*\\.pdf$",
    "path": "Documents/PDFs"
  }
]
```
> **Note-1:** As of now, the CLI doesn't support shell-type glob-patterns (like `*.pdf`, etc.). You must write proper regex patterns only (as given in the default `config.json`).

> **Note-2:** Remember to escape backslashes in JSON (e.g., use `\\.`).

> **Note-3:** Upon first installation, `config.json` will contain sample directory path (`test-forg-dir`). You must edit this with proper directory path of your choice.

---

## Usage

| Option | Shorthand | Description |
| :--- | :--- | :--- |
| `--init` | `-i` | Initialise the utility and create a default config file. |
| `--exec <DIR_PATH>`| `-e` | Organise files in the specified directory DIR_PATH (**relative to home**). |
| `--dry-run` | `-d` | Recommended: Preview matches without moving files. |
| `--allow-hidden` | | Process files starting with '.'. Use with caution. |
| `--ignore-case` | | Enable case-insensitive regex matching. |

---

> [!CAUTION]
> ### SAFETY DISCLAIMER
> Moving system files can be destructive. **forg** defaults to ignoring hidden files and provides a **dry-run** mode. Always verify your regex patterns before running execution in production environments.

---

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
