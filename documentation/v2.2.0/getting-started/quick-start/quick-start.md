# Quick Start

Clean your workspace in minutes! Follow this simple guide to start organizing with **forg**.

## Step 1: Initialize

First, set up your global configuration file in `~/.forg/config.json`:

```bash
forg init
```

This creates a default set of rules that sort common files (PDFs, Pictures, Music, etc.) into standard folders.

## Step 2: Dry Run (The Safety Step)

Before moving any files, it's always recommended to see what *would* happen:

```bash
# Preview what would happen to your Downloads folder
forg Downloads --dry-run
```

**forg** will scan your `Downloads` directory and show you which files match which rules and where they would be moved **(without actually moving them)**.

## Step 3: Execute

Ready? Run the command without the `--dry-run` flag to actually move the files:

```bash
forg Downloads
```

## Step 4: Check the Report

After execution, **forg** generates a detailed report. It tells you:
*   Total files scanned.
*   Total matched rules.
*   Total size moved.
*   How fast it was!

## Quick Tips

### Case-Insensitive Matching
Want your regex rules to ignore case? Use the `--ignore-case` flag:
```bash
forg Downloads --ignore-case
```

### Dealing with Hidden Files
By default, **forg** ignores files starting with a `.`. To include them:
```bash
forg Downloads --allow-hidden
```

### Show the File List
To see every single file being processed, use the `-L` (or `--file-list`) flag:
```bash
forg Downloads -L
```

### Decide overwrite strategy
In cases where the filename already exists in the destination directory:
```bash
forg Downloads --on-conflict skip # skips moving those files
# OR
# forg Downloads --on-conflict versioned # creates a versioned file ('_v2', '_v3', etc. suffixes)
# OR
# forg Downloads --on-conflict replace # overwrites the file with the new one
```
