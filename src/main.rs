use clap::{Parser, Subcommand};
use regex::{Regex, RegexBuilder};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    env, fs,
    io::{Error, ErrorKind, Result},
    path::PathBuf,
    process::{self, Command},
    time::Instant,
};

mod execution_report;
use execution_report::{ExecutionReport, generate_execution_report};

#[derive(Subcommand)]
enum SubCommand {
    Init,
    Uninstall,
    SelfUpdate,
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

const INSTALL_COMMAND: &str =
    "curl -sSL https://raw.githubusercontent.com/Abhijeet-Gautam5702/forg/main/install.sh | bash";

use std::io::{stderr, IsTerminal};

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

pub fn run() -> Result<()> {
    let version = env!("CARGO_PKG_VERSION");

    let cli = Cli::parse();

    // Get home directory and config path
    let home = env::home_dir()
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "Home directory not found"))?;
    let mut forg_dir_path = home.clone();
    forg_dir_path.push(".forg");
    let config_path = forg_dir_path.join("config.json");

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
                    println!("Config created at {}\n", config_path.display());
                    report_note!(
                        "Files will be moved to standard folders like ~/Pictures, ~/Documents, etc. (as defined in {})",
                        config_path.display()
                    );
                    println!("You can edit the config.json to customize your rules")
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

        // Print Execution Modes & Options (UX)
        println!("------------------------------------------------------------------");
        let mode = if cli.pattern.is_some() && cli.dest.is_some() {
            "On-The-Fly"
        } else {
            "Complete"
        };
        println!("EXECUTION MODE:");
        if cli.pattern.is_some() && cli.dest.is_some() {
            println!(
                " - {}: This will bypass the config.json and use the pattern & destination provided by the user.",
                mode
            );
        } else {
            println!(
                " - {}: This will use the configuration defined in {}",
                mode,
                config_path.display()
            );
        }

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
        }
        println!("------------------------------------------------------------------");

        let mut rules: Vec<(Regex, PathBuf)> = Vec::new();

        // ON-THE-FLY MODE
        if let (Some(p), Some(d)) = (&cli.pattern, &cli.dest) {
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
        // COMPLETE EXECUTION MODE
        // works on all the rules (pattern & destination path) provided in config.json
        else {
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

        // Initialize report metrics
        let mut total_files_scanned = 0;
        let mut total_matched = 0;
        let mut total_size_moved = 0;

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
            println!("Files to be moved:");

            for (_, filename) in filenames.iter().enumerate() {
                let from_path = target_folder_path.join(filename);
                let to_path = dest_dir.join(filename);

                if cli.dry_run {
                    // DRY-RUN MODE:
                    if to_path.exists() {
                        failed_files.push((
                            filename.clone(),
                            "filename already exists in destination".to_string(),
                        ));
                        total_skipped_conflict += 1;
                        continue;
                    }

                    println!(" - {}", filename);
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

                    // Overwrite protection:
                    // skip moving file if the filename already exists in destination
                    if to_path.exists() {
                        failed_files.push((
                            filename.clone(),
                            "filename already exists in destination".to_string(),
                        ));
                        total_skipped_conflict += 1;
                        continue;
                    }

                    // Get metadata for size tracking BEFORE moving
                    let file_size = fs::metadata(&from_path).map(|m| m.len()).unwrap_or(0);

                    match fs::rename(&from_path, &to_path) {
                        Ok(_) => {
                            println!(" - {}", filename);
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
