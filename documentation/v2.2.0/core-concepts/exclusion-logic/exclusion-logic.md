# Exclusion Logic

Beyond the standard matching rules, **forg** implements a sophisticated multi-layered exclusion system to protect sensitive data.

## The Secondary Configuration Layer: `.ignore.json`

Located at `~/.forg/.ignore.json`, this file defines a set of "Hard Exclusions." These are regex patterns for files that **must never be touched** during any organization operation.

### Technical Implementation

*   **RegexSet Integration**: **forg** utilizes a `RegexSetBuilder` to compile all patterns in `.ignore.json` into a single, highly-optimized matching object.
*   **Phase 0 Exclusion**: Files matching these patterns are filtered out during the initial scanning phase. They are not even considered for rule matching in `config.json`.
*   **Global Modifier Inheritance**: The `.ignore.json` patterns automatically inherit the state of the `--ignore-case` flag.

## Hidden File Guard

The "Hidden File Guard" is the first line of defense.
*   **Standard Behavior**: Any file starting with a dot (`.`) is ignored by default.
*   **Override**: Use `--allow-hidden` to process these files.

{WARN type="admonition" title="Critical Safety"}
While `--allow-hidden` can be used to organize configuration files, the `.ignore.json` layer remains active. This allows you to safely process hidden files while still protecting critical system files like `.ssh/config` or `.bashrc` by explicitly listing them in the ignore file.
{/WARN}
