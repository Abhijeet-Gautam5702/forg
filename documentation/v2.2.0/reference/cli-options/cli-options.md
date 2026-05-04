# CLI Options Reference

A complete guide to every command and flag available in **forg**.

## Commands

| Command | Description |
| :--- | :--- |
| `init` | Initialise **forg** and create a default `~/.forg/config.json`. |
| `uninstall` | Remove the **forg** binary and all configuration files. |
| `self-update` | Update **forg** to the latest version. |

## Positional Arguments

| Argument | Description |
| :--- | :--- |
| `[TARGET_DIR]` | The directory to organize (relative to your Home directory). |

## Options & Flags

| Option | Shorthand | Description | Default |
| :--- | :--- | :--- | :--- |
| `--dry-run` | `-d` | Preview changes without moving files. | `false` |
| `--on-conflict` | `-c` | Conflict resolution mode: `skip`, `replace`, `versioned`. | `skip` |
| `--allow-hidden` | | Process files starting with `.`. | `false` |
| `--ignore-case` | | Make regex matching case-insensitive. | `false` |
| `--file-list` | `-L` | Show real-time transformation mapping (e.g., `old -> new`) as files are processed. | `false` |
| `--pattern` | `-p` | Define a one-off regex pattern (requires `-t`). | N/A |
| `--dest` | `-t` | Define a one-off destination directory (requires `-p`). | N/A |
| `--help` | `-h` | Print help information. | N/A |
| `--version` | `-V` | Print version information. | N/A |

## Technical Implementation Details

### Global Regex Modifiers
The `--ignore-case` flag acts as a global modifier. It overrides the default case-sensitive behavior of the Rust regex engine for **all** active patterns, including those defined in `config.json`, the on-the-fly `--pattern`, and the secondary `.ignore.json` layer.


## Environment Variables

**forg** relies on the `$HOME` environment variable to resolve paths. Ensure it is correctly set (standard on almost all Unix systems).
