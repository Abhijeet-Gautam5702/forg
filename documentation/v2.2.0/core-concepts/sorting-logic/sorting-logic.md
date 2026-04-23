# Sorting Logic

Understanding how **forg** decides where to move your files is key to building an effective configuration.

## The Rule Matcher

When **forg** scans a directory, it processes each file through your list of rules in the order they appear in `config.json`.

1.  **Iterate Rules**: For each file, **forg** looks at the first rule in the config.
2.  **Regex Match**: It checks if the filename matches the `pattern` defined in that rule.
3.  **First Match Wins**: 
    *   If the file matches, it is assigned to the destination `path` of that rule.
    *   **Crucially**, **forg** then stops looking at any further rules for that file.
    *   If it doesn't match, it moves to the next rule.

## Why "First Match Wins"?

This logic allows you to create specific overrides for general rules.

### Example Scenario

Imagine you want all images to go to `Pictures`, but you want screenshots specifically to go to `Pictures/Screenshots`.

```json
[
  {
    "pattern": ".*Screenshot.*",
    "path": "Pictures/Screenshots"
  },
  {
    "pattern": ".*\\.(png|jpg|jpeg)$",
    "path": "Pictures"
  }
]
```

*   A file named `Screenshot_2023.png` matches the first rule. It goes to `Pictures/Screenshots`. The second rule is never even checked.
*   A file named `vacation.png` does *not* match the first rule, so it moves to the second. It matches and goes to `Pictures`.

{WARN type="admonition" title="Ordering Matters"}
If you swapped the rules above, `Screenshot_2023.png` would match the general image rule first and end up in `Pictures`, never reaching the specific screenshot rule!
{/WARN}

## Deterministic Execution & Grouping

**forg** performs a deterministic, two-phase execution to ensure predictable behavior and high performance:

1.  **Scanning Phase**: The tool performs a global scan of the target directory, matching files against rules and constructing a lexicographically sorted map of operations using a `BTreeMap`.
2.  **Movement Phase**: Movement is executed in groups based on the destination directory.

This architecture explains why execution reports are organized by destination and ensures that file operations are processed in a consistent order regardless of the underlying filesystem's directory entry sequence.

## Unmatched Files

If a file doesn't match any rules in your configuration, **forg** simply leaves it where it is. It won't be moved, and it won't be reported as an error. It will, however, be counted in the "Total Files" scanned in the execution report.
