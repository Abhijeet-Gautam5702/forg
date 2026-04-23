# On-the-fly Mode

Sometimes you just want to organize a single type of file without messing with your global `config.json`. **forg** handles this with **On-the-fly Mode**.

## How it Works

When you provide both a `--pattern` (`-p`) and a `--dest` (`-t`) flag, **forg** bypasses your configuration file entirely and uses only the rule you provided on the command line.

```bash
# Move all PNG files from Downloads to ~/Documents/Images/PNGs
forg Downloads -p '.*\.png$' -t Documents/Images/PNGs
```

## Requirements

1.  **Both flags required**: You cannot use `-p` without `-t`, and vice versa.
2.  **Relative Target Dir**: The `[TARGET_DIR]` (e.g., `Downloads`) is relative to home.
3.  **Relative Destination**: The destination (`-t`) is also relative to home.
4.  **Single Quotes**: Always wrap your regex pattern in single quotes `' '` to prevent your shell from interpreting special characters.

## Use Cases

*   **Project Cleanup**: Quickly move all `.log` files to a logs folder.
*   **Media Sorting**: Isolate all `.mp4` files into a specific movie folder.
*   **Code Organization**: Move all `.rs` files to a source repository.

{TIP type="admonition" title="Dry Run First"}
Just like standard mode, On-the-fly mode supports `--dry-run`. It's highly recommended to use it to verify your one-off regex pattern!
{/TIP}
