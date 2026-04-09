# 🦀 forg

[![Built with Rust](https://img.shields.io/badge/Built_with-Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**forg** is a high-performance, regex-powered file organization CLI tool built in Rust. It automates the tedious task of sorting files into designated directories based on custom pattern-matching rules, ensuring your workspace remains clean and structured.

---

## Features

- **Regex-Powered Matching**: Use the full power of Rust's regex engine to define complex file-sorting rules.
- **Priority-Based Sorting**: Rules are processed in the order they appear in your config—the first match wins.
- **Safety First**: 
  - **Dry Run Mode**: Preview exactly what will happen before moving a single byte.
  - **Overwrite Protection**: Never lose data; forg refuses to overwrite existing files in the destination.
  - **Hidden File Guard**: System and hidden files (starting with '.') are ignored by default.
- **Auto-Directory Creation**: Missing destination folders are created on-the-fly only when a match is found.
- **Optimized Execution**: Regex patterns are pre-compiled once to ensure maximum throughput even in directories with thousands of files.

---

## Local Setup

### 1. Installation
Ensure you have [Rust and Cargo](https://rustup.rs/) installed, then clone and build:
```bash
git clone https://github.com/yourusername/forg.git
cd forg
cargo build --release
```

### 2. Initialization
Create your global configuration file in `~/.forg/config.json`:
```bash
forg --init
```

### 3. Organize
Sort your Downloads folder (preview first!):
```bash
forg --exec Downloads --dry-run
```

---

## Configuration

The configuration lives at `~/.forg/config.json`. Rules are matched from **top to bottom**.

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
> **Note:** Remember to escape backslashes in JSON (e.g., use `\\.`).

---

## Usage

| Option | Shorthand | Description |
| :--- | :--- | :--- |
| `--init` | `-i` | Initialise the utility and create a default config file. |
| `--exec <DIR_PATH>`| `-e` | Organise files in the specified directory (relative to home). |
| `--dry-run` | `-d` | Recommended: Preview matches without moving files. |
| `--allow-hidden` | | Process files starting with '.'. Use with caution. |
| `--ignore-case` | | Enable case-insensitive regex matching. |

---

> [!CAUTION]
> ### Safety Disclaimer
> Moving system files can be destructive. **forg** defaults to ignoring hidden files and provides a **dry-run** mode. Always verify your regex patterns before running execution in production environments.

---

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
