# Conflict Resolution

What happens when a file you're moving already exists in the destination folder? **forg** provides three powerful ways to handle these collisions.

You can specify the conflict resolution mode using the `--on-conflict` (or `-c`) flag.

## 1. Skip (Default)

The safest option. If a file with the same name exists in the destination, **forg** will skip the move and report it in the execution summary.

```bash
forg Downloads --on-conflict skip
```

## 2. Replace

Use this if you want to overwrite the existing file. However, **forg** still prioritizes safety by creating a `.bak` backup of the original file in the destination before replacing it.

{WARN type="admonition" title="One-Generation Backup"}
The `replace` mode implements a destructive-backup strategy. If a `.bak` file already exists for the target filename, it is deleted before the current destination file is rotated into the backup slot. Only the most recent version is preserved as a backup.
{/WARN}

```bash
forg Downloads --on-conflict replace
```

*Example:* If `image.png` exists in `~/Pictures`, **forg** will:
1. Rename the existing `~/Pictures/image.png` to `~/Pictures/image.png.bak`.
2. Move the new `image.png` from `Downloads` to `~/Pictures`.

## 3. Versioned

This is perfect for keeping multiple versions of the same file. **forg** will automatically increment a version number in the filename.

```bash
forg Downloads --on-conflict versioned
```

*Logic:*
*   If `report.pdf` exists, the new file becomes `report_v1.pdf`.
*   If `report_v1.pdf` also exists, it becomes `report_v2.pdf`.
*   And so on.

{IMPORTANT type="admonition" title="Dry Run Support"}
All conflict resolution modes are fully supported in `--dry-run`. You will see exactly how a file would be renamed or if it would be skipped before any changes are made.
{/IMPORTANT}
