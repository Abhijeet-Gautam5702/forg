# Changelog
## v2.2.0
### Added
- New `--on-conflict` flag: Decide operation in case of filename conflicts (skip/versioned-name/overwrite)

## v2.1.0
### Added
- Manual and E2E Tests for Config-based execution
- New `--file-list` flag: Show list of files being processed
- `.ignore.json` to avoid sensitive files from processing by default
### Improved
- UX: Execution Report for detailed metrics (throughput, time taken, volume moved, etc.)
- UX: hide list of file(s) to be processed by default (can be seen by the new `-L` flag)
- default_config.json
### Fixed
- ANSI Color-coded issue in test outputs

## v2.0.1
### Improved
- UX: color-coded error messages for better readability
- UX: display time (in ms) taken to move files
- UX: display forg version number on update/install/uninstall

## v2.0.0
### Added
- `forg self-update` & `forg uninstall` commands
- support for `forg <TARGET_DIR> --pattern <PATTERN> --dest <DEST_PATH>` comamnd (On-The-Fly Execution Mode)
### Improved
- `default_config.json` covering all general file types and cases
