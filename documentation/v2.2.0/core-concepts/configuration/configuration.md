# Configuration

**forg** is controlled by a simple JSON configuration file located at `~/.forg/config.json`.

## Structure

The configuration is an array of objects. Each object represents a rule with two properties:
1.  `pattern`: A Rust-flavor regex pattern to match filenames.
2.  `path`: The destination directory (relative to your Home directory).

### Example `config.json`

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

## JSON-Specific Rules

### Escaping Backslashes
In JSON, the backslash `\` is an escape character. Since regex often uses backslashes (e.g., `\.` to match a literal dot), you must double-escape them:
*   Use `\\.pdf` instead of `\.pdf`.

### No Glob Support
As of now, **forg** does not support shell-type glob-patterns (like `*.pdf`, etc.). You must use proper regex patterns (as shown in the examples) to define your rules.

### Pathing
*   **Relative to Home**: All paths in `config.json` are automatically prefixed with your Home directory (`~`).
*   **No Leading Slash**: Do *not* start your path with a `/`.
    *   ✅ `Pictures`
    *   ❌ `/Pictures`

## Priority

Rules are matched from **top to bottom**. The first rule that matches a file's name determines where that file will be moved. Once a match is found, **forg** stops evaluating further rules for that specific file.

{NOTE type="admonition" title="Pro Tip"}
Place your most specific rules at the top of the file and more general catch-all rules at the bottom.
{/NOTE}

## Default Rules

Upon running `forg init`, a default configuration is generated that covers most common file types:
*   **Pictures**: Screenshots, PNG, JPG, etc.
*   **Videos**: MP4, MKV, MOV, etc.
*   **Music**: MP3, WAV, FLAC, etc.
*   **Documents**: PDFs, DOCX, TXT, CSV, etc.
*   **Archives**: ZIP, TAR, RAR, etc.
*   **Installers**: DMG, PKG, EXE, etc.
