use clap::Parser;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env, fs,
    io::{Error, ErrorKind, Result},
    path::PathBuf,
};

#[derive(Parser)]
#[command(name = "forg")]
#[command(version = "1.0")]
#[command(about = "Organises files into designated folders automatically", long_about = None)]
struct Cli {
    #[arg(long)]
    init: bool,
    #[arg(long)]
    exec: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigItem {
    pattern: String,
    path: String,
}

pub fn main() -> Result<()> {
    let cli = Cli::parse();

    // Get home directory and config path once
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

        // Read and parse config
        let config_data_str = fs::read_to_string(&config_path)?;
        let config_json: Vec<ConfigItem> = serde_json::from_str(&config_data_str).map_err(|e| {
            Error::new(ErrorKind::InvalidData, format!("Config parse error: {}", e))
        })?;

        // Pre-compile directory map for performance
        let mut directory_map: HashMap<String, (Regex, PathBuf)> = HashMap::new();
        for config_item in &config_json {
            let folder_complete_path = home.join(&config_item.path);
            let pattern_regex = Regex::new(&config_item.pattern).map_err(|e| {
                Error::new(
                    ErrorKind::InvalidInput,
                    format!("Invalid regex '{}': {}", config_item.pattern, e),
                )
            })?;
            directory_map.insert(
                config_item.pattern.clone(),
                (pattern_regex, folder_complete_path),
            );
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

        println!("Scanning: {}", target_path.display());

        let entries = fs::read_dir(target_path)?;
        let mut moved_count = 0;

        for entry in entries {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                if let Some(filename_str) = entry.file_name().to_str() {
                    for (_, (pattern_regex, dest_dir)) in &directory_map {
                        if pattern_regex.is_match(filename_str) {
                            // Optimization: Only create directory when a match is actually found
                            if !dest_dir.exists() {
                                fs::create_dir_all(dest_dir)?;
                            }

                            let from_path = entry.path();
                            let to_path = dest_dir.join(filename_str);

                            println!("Moving: {} -> {}", filename_str, dest_dir.display());
                            fs::rename(from_path, to_path)?;
                            moved_count += 1;
                            break; // Bug Fix: Stop checking other patterns once file is moved
                        }
                    }
                }
            }
        }
        println!("Done! Moved {} files.", moved_count);
    } else {
        println!("Please provide an option. Use --help for info.");
    }

    Ok(())
}
