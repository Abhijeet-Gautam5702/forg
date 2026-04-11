use clap::{Parser, Subcommand};
use regex::{Regex, RegexBuilder};
use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    io::{Error, ErrorKind, Result},
    path::PathBuf,
    process::Command,
};

#[derive(Subcommand)]
enum SubCommand {
    Init,
    Uninstall,
    SelfUpdate,
}

#[derive(Parser)]
#[command(name = "forg")]
#[command(version = "0.1.5")]
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

pub fn main() -> Result<()> {
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
                if !forg_dir_path.exists() {
                    fs::create_dir_all(&forg_dir_path)?;
                }

                if !config_path.exists() {
                    fs::write(&config_path, DEFAULT_CONFIG)?;
                    println!("Initialised: Config created at {}", config_path.display());
                    println!(
                        "NOTE: Unless you edit {}, all the moved files will go to ~/test-forg-dir/ (see {})",
                        config_path.display(),
                        config_path.display()
                    );
                    println!("So kindly edit the config.json according to your needs")
                } else {
                    println!("Already initialised at {}", config_path.display());
                }
            }
            SubCommand::Uninstall => {
                println!("Uninstalling forg...");
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
                        println!("Uninstall Done!")
                    }
                    Err(e) => {
                        // binary might be installed globally (/usr/local/bin)
                        if e.kind() == std::io::ErrorKind::PermissionDenied {
                            println!("[ERROR] Uninstall Failed: PERMISSION DENIED");
                            println!("Try running with sudo:");
                            println!("  sudo forg uninstall");
                        } else {
                            println!("[ERROR] Uninstall Failed: {}", e);
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
                            println!("forg updated successfully");
                        } else {
                            println!("[ERROR] Update script failed with status: {}", status);
                        }
                    }
                    Err(e) => {
                        if e.kind() == ErrorKind::PermissionDenied {
                            println!("[ERROR] Failed to update forg: PERMISSION DENIED");
                            println!("Try running with sudo:");
                            println!("  sudo forg self-update");
                        } else {
                            println!("[ERROR] Failed to update forg: {}", e);
                        }
                    }
                }
            }
        }
        return Ok(()); // early return if any sub command is triggered
    }

    // EXECUTION
    if let Some(target_dir) = cli.target_dir {
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

        let mut rules: Vec<(Regex, PathBuf)> = Vec::new();

        // ON-THE-FLY MODE
        // works only on a single pattern and destination path provided by the user
        if let (Some(p), Some(d)) = (cli.pattern, cli.dest) {
            println!(
                "ON-THE-FLY MODE: \nBypassing config.json\nUsing user-provided pattern and destination path..."
            );
            let dest_folder_path = home.join(&d);
            let pattern_regex = RegexBuilder::new(&p)
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
                    "Utility not initialised. Run 'forg init' first.",
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

        // FILE MOVING LOGIC
        println!("\nScanning: {}", target_folder_path.display());
        println!(
            "\nNOTE: During actual file-moving (dry-run disabled) any missing directory (in destination directory) will be auto-created.\n"
        );

        let entries = fs::read_dir(target_folder_path)?;
        let mut moved_count = 0;
        let mut failed_files: Vec<(String, String)> = Vec::new();
        for entry in entries {
            let entry = entry?;
            if entry.file_type()?.is_file() {
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

                            // skip same directory move
                            if to_path == from_path {
                                break;
                            }

                            if cli.dry_run {
                                println!(
                                    "{}) Would move: {} -> {}",
                                    moved_count + 1,
                                    filename_str,
                                    dest_dir.display()
                                );
                                moved_count += 1;
                            }
                            // move to desstination folder
                            else {
                                // Optimization: Only create directory when a match is actually found
                                if !dest_dir.exists() {
                                    if let Err(e) = fs::create_dir_all(dest_dir) {
                                        failed_files.push((
                                            filename_str.to_string(),
                                            format!("Failed to create dir: {}", e),
                                        ));
                                        break;
                                    }
                                }

                                // File overwrite protection:
                                // skip file moving if a file with same name already exists in destination
                                if to_path.exists() {
                                    failed_files.push((
                                        filename_str.to_string(),
                                        String::from("filename already exists in destination"),
                                    ));
                                    continue;
                                }
                                match fs::rename(&from_path, &to_path) {
                                    Ok(_) => {
                                        println!(
                                            "{}) {} -> {}",
                                            moved_count + 1,
                                            filename_str,
                                            dest_dir.display()
                                        );
                                        moved_count += 1;
                                    }
                                    Err(e) => {
                                        failed_files.push((filename_str.to_string(), e.to_string()))
                                    }
                                }
                            }

                            break; // Stop checking other patterns once the first match is found (Priority)
                        }
                    }
                }
            }
        }

        if cli.dry_run {
            println!("\nTotal Files To Be Moved: {}", moved_count);
        } else {
            println!("\nTotal Files Moved: {}", moved_count);
            if !failed_files.is_empty() {
                println!("\nErrors occurred for the following files:");
                for (file, error) in failed_files {
                    println!(" - {}: {}", file, error);
                }
            }
        }
    }
    // UNKNOWN OPERATION
    else {
        println!("Please provide an option. Use --help for info.");
    }

    Ok(())
}
