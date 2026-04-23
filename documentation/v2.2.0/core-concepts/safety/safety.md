# Safety First

**forg** is designed with the philosophy that your data is sacred. We provide multiple layers of protection to ensure that organizing your files never results in losing them.

## 1. Dry Run Mode

The most important safety feature is the `--dry-run` flag. It allows you to simulate the entire organization process without touching a single byte on your disk.

```bash
forg Downloads --dry-run
```

We recommend always running a dry run after modifying your `config.json` or when using On-the-fly mode for the first time.

## 2. Overwrite Protection

By default, **forg** will *never* silently overwrite an existing file. If a collision is detected at the destination, **forg** will skip the file unless you explicitly tell it otherwise using the `--on-conflict` flag.

## 3. Automatic Backups

Even when you choose to `replace` files on conflict, **forg** still has your back. Before replacing a file, it renames the original to `[filename].bak`. This ensures that if you accidentally overwrite something, you can still recover the original from the backup.

## 4. Hidden File Guard

System files and configuration folders often start with a dot (`.`). Moving these accidentally can break applications or even your OS. **forg** ignores these files by default.

To process them, you must use the explicit `--allow-hidden` flag, serving as a "mental speed bump" for potentially dangerous operations.

## 5. Regex Validation

**forg** validates your regex patterns at startup. If you have an invalid pattern in your `config.json`, the tool will refuse to run and will point you to the exact rule that needs fixing, preventing unpredictable matching behavior.

{DANGER type="admonition" title="SAFETY DISCLAIMER"}
Moving system files can be destructive. **forg** defaults to ignoring hidden files and provides a **dry-run** mode. Always verify your regex patterns before running execution in production environments.
{/DANGER}
