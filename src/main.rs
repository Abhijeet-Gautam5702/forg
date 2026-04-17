use clap::{Parser, Subcommand, ValueEnum};
use regex::{Regex, RegexBuilder, RegexSetBuilder};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    env, fs,
    io::{Error, ErrorKind, IsTerminal, Result, stderr},
    path::{Path, PathBuf},
    process::{self, Command},
    time::Instant,
};

mod execution_report;
use execution_report::{ExecutionReport, generate_execution_report};

mod history;

#[derive(Subcommand)]
enum SubCommand {
    Init,
    Uninstall,
    SelfUpdate,
}

enum ExecutionMode {
    ConfigBypass,
    ConfigBased,
}

#[derive(ValueEnum, Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum OnConflictOperation {
    Skip,
    Replace,
    Versioned,
}

#[derive(Parser)]
#[command(name = "forg")]
#[command(version)]
#[command(
    about = "A high-performance, regex-powered file organization tool.",
    long_about = "forg is a command-line utility that automates directory organization using regex-based rules. It scans target directories and moves files to designated folders based on a priority-ordered configuration. Key features include a safety-first dry-run mode, overwrite protection, case-insensitive matching, and optional processing of hidden files.",
    override_usage = "forg [TARGET_DIR] [OPTIONS] [COMMAND]"
)]
struct Cli {
    // The directory to organise
    target_dir: Option<String>,

    #[arg(
        short('d'),
        long,
        default_value_t = false,
        help = "Preview the changes without moving any files"
    )]
    dry_run: bool,

    #[arg(
        long,
        default_value_t = false,
        help = "**POTENTIALLY SYSTEM BREAKING**: Allow processing of hidden files (starting with '.')"
    )]
    allow_hidden: bool,

    #[arg(
        long,
        default_value_t = false,
        help = "Make regex pattern matching case-insensitive"
    )]
    ignore_case: bool,

    #[arg(
        short('L'),
        long,
        default_value_t = false,
        help = "Show the list of files being processed"
    )]
    file_list: bool,

    #[arg(
        short('c'),
        long,
        value_enum,
        default_value_t = OnConflictOperation::Skip,
        help = "How to handle name conflicts while moving files to the destination directory"
    )]
    on_conflict: OnConflictOperation,

    #[arg(long, short('p'), requires = "dest", help = "Define a regex pattern")]
    pattern: Option<String>,

    #[arg(
        long,
        short('t'),
        requires = "pattern",
        help = "Define a destination directory (relative to home)"
    )]
    dest: Option<String>,

    #[command(subcommand)]
    sub_command: Option<SubCommand>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigItem {
    pattern: String,
    path: String,
}

// Template (default config)
// Note: We cannot directly read from the default_config.json file
// as rust compiler doesn't bundle the file with the binary
// so we embed the file contents into DEFAULT_CONFIG at compile time
const DEFAULT_CONFIG: &str = include_str!("../default_config.json");
const DEFAULT_IGNORE: &str = include_str!("../default_ignore.json");

const INSTALL_COMMAND: &str =
    "curl -sSL https://raw.githubusercontent.com/Abhijeet-Gautam5702/forg/main/install.sh | bash";

// MACROS
macro_rules! report_err {
    ($($arg:tt)*) => {{
        if stderr().is_terminal() {
            eprintln!("\x1b[31;1mERROR:\x1b[0m {}", format_args!($($arg)*));
        } else {
            eprintln!("ERROR: {}", format_args!($($arg)*));
        }
    }};
}

macro_rules! report_note {
    ($($arg:tt)*) => {{
        if stderr().is_terminal() {
            eprintln!("\x1b[33;1mNOTE:\x1b[0m {}", format_args!($($arg)*));
        } else {
            eprintln!("NOTE: {}", format_args!($($arg)*));
        }
    }};
}
macro_rules! warn {
    ($($arg:tt)*) => {{
        if stderr().is_terminal() {
            eprintln!("\x1b[33;1m{}\x1b[0m", format_args!($($arg)*));
        } else {
            eprintln!("{}", format_args!($($arg)*));
        }
    }};
}

/// Handles file naming in case of conflict by versioning it.
/// Logic: sample_txt_v23.md -> sample_txt_v24.md
/// It looks for '_v' followed by digits at the end of the file stem.
/// If not found, it appends '_v1'.
pub fn get_versioned_name(filename: &str) -> String {

    let path = Path::new(filename);
    let filename_wo_extension = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");

    // Traverse the filename from the end until '_v' is found
    if let Some(v_index) = filename_wo_extension.rfind("_v") {
        // version_part for sample_variable_v12.txt will be '12'
        let version_part = &filename_wo_extension[v_index + 2..];
        // If '_v' is followed by digits till the end, then increment the version
        if !version_part.is_empty() && version_part.chars().all(|c| c.is_ascii_digit()) {
            if let Ok(version) = version_part.parse::<u32>() {
                let versioned_filename_wo_extension =
                    format!("{}_v{}", &filename_wo_extension[..v_index], version + 1);
                return if extension.is_empty() {
                    versioned_filename_wo_extension
                } else {
                    format!("{}.{}", versioned_filename_wo_extension, extension)
                };
            }
        }
    }

    // Otherwise, simply add '_v1' at the end of the filename (before extension).
    let versioned_filename_wo_extension = format!("{}_v1", filename_wo_extension);
    if extension.is_empty() {
        versioned_filename_wo_extension
    } else {
        format!("{}.{}", versioned_filename_wo_extension, extension)
    }
}

/// Handles the 'replace' conflict resolution by backing up the existing file.
/// Logic: Check if [filename].bak is present: YES => delete it.
/// Then rename the existing file to [filename].bak.
pub fn handle_replace_conflict(to_path: &Path) -> Result<()> {
    let mut bak_path_str = to_path.to_string_lossy().to_string();
    bak_path_str.push_str(".bak");
    let bak_path = PathBuf::from(bak_path_str);

    if bak_path.exists() {
        fs::remove_file(&bak_path)?;
    }
    fs::rename(to_path, &bak_path)?;
    Ok(())
}

pub fn run() -> Result<()> {
    let version = env!("CARGO_PKG_VERSION");

    let cli = Cli::parse();

    // Get home directory and config path
    let home = env::home_dir()
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "Home directory not found"))?;
    let mut forg_dir_path = home.clone();
    forg_dir_path.push(".forg");
    let config_path = forg_dir_path.join("config.json");
    let ignore_path = forg_dir_path.join(".ignore.json");

    // SUB COMMANDS
    if let Some(sub_c) = cli.sub_command {
        match sub_c {
            SubCommand::Init => {
                println!("Initialising forg v{}", version);
                if !forg_dir_path.exists() {
                    fs::create_dir_all(&forg_dir_path)?;
                }

                if !config_path.exists() {
                    fs::write(&config_path, DEFAULT_CONFIG)?;

                    if !ignore_path.exists() {
                        println!("\nCreating .ignore.json...");
                        fs::write(&ignore_path, DEFAULT_IGNORE)?;
                        println!(".ignore.json created: {}", ignore_path.display());
                        warn!(
                            "\nIMPORTANT:\n.ignore.json defines regex patterns for files that must never be touched while file-moving operation.\nBe careful while editing it!\n"
                        )
                    }
                    println!(
                        "config.json created: {}\nEdit to customise your rules.",
                        config_path.display()
                    );
                } else {
                    println!("Already initialised at {}", config_path.display());
                }
            }
            SubCommand::Uninstall => {
                println!("Uninstalling forg v{}...", version);
                // remove config file
                if forg_dir_path.exists() {
                    println!("Removing dir: {}", forg_dir_path.display());
                    fs::remove_dir_all(&forg_dir_path)?;
                }
                // remove binary
                let binary_path = env::current_exe().unwrap();
                println!("Removing binary: {}", binary_path.display());
                match fs::remove_file(&binary_path) {
                    Ok(()) => {
                        println!("forg v{} uninstalled successfully", version);
                    }
                    Err(e) => {
                        // binary might be installed globally (/usr/local/bin)
                        if e.kind() == std::io::ErrorKind::PermissionDenied {
                            report_err!("Uninstall Failed: PERMISSION DENIED");
                            println!("Try running with sudo:");
                            println!("  sudo forg uninstall");
                        } else {
                            report_err!("Uninstall Failed: {}", e);
                        }
                    }
                }
            }
            SubCommand::SelfUpdate => {
                println!("Updating forg...");
                // spawn a new shell process and install via script
                match Command::new("sh").arg("-c").arg(&INSTALL_COMMAND).status() {
                    Ok(status) => {
                        if status.success() {
                            println!("forg updated to version");
                        } else {
                            report_err!("Update script failed with status: {}", status);
                        }
                    }
                    Err(e) => {
                        if e.kind() == ErrorKind::PermissionDenied {
                            report_err!("Failed to update forg: PERMISSION DENIED");
                            println!("Try running with sudo:");
                            println!("  sudo forg self-update");
                        } else {
                            report_err!("Failed to update forg: {}", e);
                        }
                    }
                }
            }
        }
        return Ok(()); // early return if any sub command is triggered
    }

    // EXECUTION
    if let Some(target_dir) = cli.target_dir {
        let now = Instant::now();

        // Check if the directory (to organise) exists
        let target_folder_path = home.join(target_dir);
        if !target_folder_path.exists() {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!(
                    "Target directory '{}' not found.",
                    target_folder_path.display()
                ),
            ));
        }

        let execution_mode = if cli.pattern.is_some() && cli.dest.is_some() {
            ExecutionMode::ConfigBypass
        } else {
            ExecutionMode::ConfigBased
        };

        // Print Execution Modes & Options (UX)
        println!("------------------------------------------------------------------");
        println!("EXECUTION MODE:");
        let mode: String = match execution_mode {
            ExecutionMode::ConfigBypass => {
                println!(
                    " - On-The-Fly: This will bypass the config.json and use the pattern & destination provided by the user."
                );
                "On-The-Fly".to_string()
            }
            ExecutionMode::ConfigBased => {
                println!(
                    " - Complete: This will use the configuration defined in {}",
                    config_path.display()
                );
                "Complete".to_string()
            }
        };

        let mut enabled_options = Vec::new();
        if cli.dry_run {
            enabled_options.push("Dry-run");
        }
        if cli.ignore_case {
            enabled_options.push("Ignore-case");
        }
        if cli.allow_hidden {
            enabled_options.push("Allow-hidden");
        }
        if cli.on_conflict != OnConflictOperation::Skip {
            enabled_options.push("On-conflict");
        }

        if !enabled_options.is_empty() {
            println!("\nOPTIONS ENABLED:");
            if cli.dry_run {
                println!(" - Dry-run: Preview mode. No files will actually be moved.");
            }
            if cli.ignore_case {
                println!(" - Ignore-case: Regex matching will be case-insensitive.");
            }
            if cli.allow_hidden {
                println!(
                    " - Allow-hidden: Hidden files (starting with '.') will also be processed."
                );
            }
            match cli.on_conflict {
                OnConflictOperation::Replace => {
                    println!(
                        " - On-conflict: Replace existing files in destination (with .bak backup of the original file)"
                    );
                }
                OnConflictOperation::Versioned => {
                    println!(" - On-conflict: Create versioned filename (e.g. _v2) on conflict");
                }
                OnConflictOperation::Skip => {}
            }
        }
        println!("------------------------------------------------------------------");

        // Initialise Rules (regex objects)
        let mut rules: Vec<(Regex, PathBuf)> = Vec::new();
        match execution_mode {
            // RULES: ON-THE-FLY MODE
            ExecutionMode::ConfigBypass => {
                let d = &cli.dest.unwrap();
                let p = &cli.pattern.unwrap();
                let dest_folder_path = home.join(d);
                let pattern_regex = RegexBuilder::new(p)
                    .case_insensitive(cli.ignore_case)
                    .build()
                    .map_err(|e| {
                        Error::new(
                            ErrorKind::InvalidInput,
                            format!("Invalid regex '{}': {}", p, e),
                        )
                    })?;
                rules.push((pattern_regex, dest_folder_path));
            }
            // RULES: COMPLETE EXECUTION MODE (CONFIG-BASED)
            ExecutionMode::ConfigBased => {
                if !config_path.exists() {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        "No config.json found. Run 'forg init' first.",
                    ));
                }

                // Read and parse config
                let config_data_str = fs::read_to_string(&config_path)?;
                let config_json: Vec<ConfigItem> =
                    serde_json::from_str(&config_data_str).map_err(|e| {
                        Error::new(ErrorKind::InvalidData, format!("Config parse error: {}", e))
                    })?;

                for config_item in &config_json {
                    let dest_folder_path = home.join(&config_item.path);
                    let pattern_regex = RegexBuilder::new(&config_item.pattern)
                        .case_insensitive(cli.ignore_case)
                        .build()
                        .map_err(|e| {
                            Error::new(
                                ErrorKind::InvalidInput,
                                format!("Invalid regex '{}': {}", config_item.pattern, e),
                            )
                        })?;
                    rules.push((pattern_regex, dest_folder_path));
                }
            }
        }

        // Initialize report metrics
        let mut total_files_scanned = 0;
        let mut total_matched = 0;
        let mut total_size_moved = 0;

        // FILE EXCLUSION LOGIC
        // load the regex set of files to be ignored
        let mut ignore_set = None;
        if ignore_path.exists() {
            let ignore_json_data_str = fs::read_to_string(&ignore_path)?;
            let ignore_json: Vec<String> = serde_json::from_str(&ignore_json_data_str)
                .map_err(|e| Error::new(ErrorKind::InvalidData, format!("Parse Error: {}", e)))?;
            let set = RegexSetBuilder::new(&ignore_json)
                .case_insensitive(cli.ignore_case)
                .build()
                .map_err(|e| {
                    Error::new(
                        ErrorKind::InvalidInput,
                        format!("Invalid regex in .ignore.json: {}", e),
                    )
                })?;
            ignore_set = Some(set);
        }

        // FILE GROUPING LOGIC
        // group the files into directories they're to be organised into
        println!("\nScanning: {}", target_folder_path.display());
        let entries = fs::read_dir(&target_folder_path)?;
        let mut grouped_moves: BTreeMap<PathBuf, Vec<String>> = BTreeMap::new();
        for entry in entries {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                total_files_scanned += 1;
                if let Some(filename_str) = entry.file_name().to_str() {
                    // Skip ignored files (from .ignore.json)
                    if let Some(set) = &ignore_set {
                        if set.is_match(filename_str) {
                            continue;
                        }
                    }

                    // Skip hidden files if not allowed
                    if !cli.allow_hidden && filename_str.starts_with('.') {
                        continue;
                    }

                    // match pattern against the rules
                    for (pattern_regex, dest_dir) in &rules {
                        if pattern_regex.is_match(filename_str) {
                            let from_path = entry.path();
                            let to_path = dest_dir.join(filename_str);

                            if to_path != from_path {
                                grouped_moves
                                    .entry(dest_dir.clone())
                                    .or_default()
                                    .push(filename_str.to_string());
                                total_matched += 1;
                            }
                            break;
                        }
                    }
                }
            }
        }

        // FILE MOVING LOGIC
        let move_start = Instant::now();
        let mut total_moved = 0;
        let mut total_skipped_conflict = 0;
        let mut failed_files: Vec<(String, String)> = Vec::new();

        for (dest_dir, filenames) in grouped_moves {
            let mut move_cnt_for_this_dest = 0;

            println!("\nDestination Directory: {}", dest_dir.display());
            if cli.file_list {
                println!("Files to be moved:");
            }

            for (_, filename) in filenames.iter().enumerate() {
                let from_path = target_folder_path.join(filename);
                let mut to_path = dest_dir.join(filename);
                let mut final_filename = filename.clone();

                if cli.dry_run {
                    // DRY-RUN MODE:
                    if to_path.exists() {
                        match cli.on_conflict {
                            OnConflictOperation::Skip => {
                                failed_files.push((
                                    filename.clone(),
                                    "filename already exists in destination (skipped)".to_string(),
                                ));
                                total_skipped_conflict += 1;
                                continue;
                            }
                            OnConflictOperation::Replace => {
                                if cli.file_list {
                                    println!(" - {} (REPLACES existing)", filename);
                                }
                            }
                            OnConflictOperation::Versioned => {
                                final_filename = get_versioned_name(filename);
                                if cli.file_list {
                                    println!(" - {} (VERSIONED as {})", filename, final_filename);
                                }
                            }
                        }
                    } else if cli.file_list {
                        println!(" - {}", filename);
                    }
                    total_moved += 1;
                    move_cnt_for_this_dest += 1;

                    // Track size in dry-run by querying metadata
                    if let Ok(metadata) = fs::metadata(&from_path) {
                        total_size_moved += metadata.len();
                    }
                }
                // Actual file moving logic
                else {
                    if !dest_dir.exists() {
                        if let Err(e) = fs::create_dir_all(&dest_dir) {
                            failed_files
                                .push((filename.clone(), format!("Failed to create dir: {}", e)));
                            continue;
                        }
                    }

                    // Overwrite protection & Conflict Handling:
                    if to_path.exists() {
                        match cli.on_conflict {
                            OnConflictOperation::Skip => {
                                failed_files.push((
                                    filename.clone(),
                                    "filename already exists in destination".to_string(),
                                ));
                                total_skipped_conflict += 1;
                                continue;
                            }
                            OnConflictOperation::Replace => {
                                // Logic for replace:
                                // Check if [filename].bak is present: YES => delete it.
                                // Then rename the existing file to [filename].bak.
                                if let Err(e) = handle_replace_conflict(&to_path) {
                                    failed_files.push((
                                        filename.clone(),
                                        format!("Failed to handle conflict (replace): {}", e),
                                    ));
                                    continue;
                                }
                            }
                            OnConflictOperation::Versioned => {
                                // rename the current file as the next version according to the destination filename
                                final_filename = get_versioned_name(filename);
                                to_path = dest_dir.join(&final_filename);
                            }
                        }
                    }

                    // Get metadata for size tracking BEFORE moving
                    let file_size = fs::metadata(&from_path).map(|m| m.len()).unwrap_or(0);

                    match fs::rename(&from_path, &to_path) {
                        Ok(_) => {
                            if cli.file_list {
                                if final_filename != *filename {
                                    println!(" - {} -> {}", filename, final_filename);
                                } else {
                                    println!(" - {}", filename);
                                }
                            }
                            total_moved += 1;
                            move_cnt_for_this_dest += 1;
                            total_size_moved += file_size;
                        }
                        Err(e) => failed_files.push((filename.clone(), e.to_string())),
                    }
                }
            }

            println!("File count: {}", move_cnt_for_this_dest);
        }
        let moving_elapsed_ms = move_start.elapsed().as_millis();

        // EXECUTION REPORT GENERATION

        // total_skipepd => file that were skipped
        // total_failed => any other errors
        let total_failed = failed_files.len() - total_skipped_conflict;

        let report = ExecutionReport {
            target_dir: target_folder_path,
            mode: mode.to_string(),
            on_conflict: format!("{:?}", cli.on_conflict).to_lowercase(),
            dry_run: cli.dry_run,
            ignore_case: cli.ignore_case,
            allow_hidden: cli.allow_hidden,
            total_files_scanned,
            total_matched,
            total_moved,
            total_skipped_conflict,
            total_failed,
            total_size_moved,
            elapsed_ms: now.elapsed().as_millis(),
            moving_elapsed_ms,
            failed_files,
        };
        generate_execution_report(report);
    }
    // UNKNOWN OPERATION
    else {
        println!("Please provide an option. Use --help for info.");
    }

    Ok(())
}

pub fn main() {
    if let Err(e) = run() {
        println!("");
        report_err!("{}", e);
        process::exit(1);
    }
}
