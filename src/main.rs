use clap::Parser;
use regex::{Regex, RegexBuilder};
use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    io::{Error, ErrorKind, Result},
    path::PathBuf,
};

#[derive(Parser)]
#[command(name = "forg")]
#[command(version = "1.0")]
#[command(about = "Organises files into designated folders automatically", long_about = None)]
struct Cli {
    #[arg(
        short,
        long,
        help = "Initialise the utility by creating a config file in ~/.forg/config.json (to be run only once in the beginning)"
    )]
    init: bool,

    #[arg(
        short,
        long,
        value_name = "DIR_PATH",
        help = "Organise files in the specified directory (relative to home)"
    )]
    exec: Option<String>,

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
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigItem {
    pattern: String,
    path: String,
}

pub fn main() -> Result<()> {
    let cli = Cli::parse();

    // Get home directory and config path
    let home = env::home_dir()
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "Home directory not found"))?;
    let mut forg_dir_path = home.clone();
    forg_dir_path.push(".forg");
    let config_path = forg_dir_path.join("config.json");

    if cli.init {
        if !forg_dir_path.exists() {
            fs::create_dir_all(&forg_dir_path)?;
        }

        if !config_path.exists() {
            let default_config_path = PathBuf::from("default_config.json");
            if !default_config_path.exists() {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    "default_config.json not found in current directory",
                ));
            }
            fs::copy(default_config_path, &config_path)?;
            println!("Initialised: Config created at {}", config_path.display());
        } else {
            println!("Already initialised at {}", config_path.display());
        }
    } else if let Some(target_dir) = cli.exec {
        if !config_path.exists() {
            return Err(Error::new(
                ErrorKind::NotFound,
                "Utility not initialised. Run 'forg --init' first.",
            ));
        }

        // Print Mode Disclaimers
        if cli.dry_run || cli.allow_hidden || cli.ignore_case {
            println!("--------------------------------------------");
            if cli.dry_run {
                println!("DRY RUN ENABLED: No files will actually be moved.");
            }
            if cli.allow_hidden {
                println!(
                    "ALLOW HIDDEN: System/hidden files (starting with '.') will be processed."
                );
            }
            if cli.ignore_case {
                println!("IGNORE CASE: Regex matching will be case-insensitive.");
            }
            println!("--------------------------------------------");
        }

        // Read and parse config
        let config_data_str = fs::read_to_string(&config_path)?;
        let config_json: Vec<ConfigItem> = serde_json::from_str(&config_data_str).map_err(|e| {
            Error::new(ErrorKind::InvalidData, format!("Config parse error: {}", e))
        })?;

        // Pre-compile rules with regex objects in a Vector to preserve order (priority preservation)
        let mut rules: Vec<(Regex, PathBuf)> = Vec::new();
        for config_item in &config_json {
            let folder_complete_path = home.join(&config_item.path);
            let pattern_regex = RegexBuilder::new(&config_item.pattern)
                .case_insensitive(cli.ignore_case)
                .build()
                .map_err(|e| {
                    Error::new(
                        ErrorKind::InvalidInput,
                        format!("Invalid regex '{}': {}", config_item.pattern, e),
                    )
                })?;
            rules.push((pattern_regex, folder_complete_path));
        }

        let target_path = home.join(target_dir);
        if !target_path.exists() {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!(
                    "Target directory (--exec <DIR_PATH>) not found: {}",
                    target_path.display()
                ),
            ));
        }

        println!("\nScanning: {}", target_path.display());

        let entries = fs::read_dir(target_path)?;
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
    } else {
        println!("Please provide an option. Use --help for info.");
    }

    Ok(())
}
