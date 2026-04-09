use clap::Parser;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env, fs,
    io::{Error, Result},
    path::PathBuf,
};

// CLI TOOL
// forg --init => create a .forg/forg.config file
// sample forg.config file
/*
 * [
 *  {
 *      "pattern": "*.png",
 *      "path": "/home/abhijeet/Pictures/Screenshots"
 *  },
 *  {
 *      "pattern": "*.txt",
 *      "path": "/home/abhijeet/Documents"
 *  },
 * ]
 *
 */
// forg --exec ./Downloads
// - scans ./Downloads directory
// - move all the files (not directories) to their respective paths
// - if directories in path don't exist, create them and then move
//

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
    // println!("init: {:?}", cli.init);
    // println!("exec: {:?}", cli.exec);

    // if init == true => create forg.config file in ~/.forg/config.json
    if cli.init == true {
        // check if the directory exists
        match env::home_dir() {
            Some(path) => {
                let mut forg_dir_path = PathBuf::from(&path);
                forg_dir_path.push(".forg");

                if !forg_dir_path.exists() {
                    // create .forg directory & default config.json
                    fs::create_dir_all(&forg_dir_path)?;

                    // config file
                    let config_path = forg_dir_path.join("config.json");
                    fs::File::create_new(&config_path)?;

                    // default config file
                    let default_config_path = PathBuf::from("default_config.json");
                    if !default_config_path.exists() {
                        panic!("default config file not exists")
                    }

                    // copy contents from default config -> config (newly created)
                    fs::copy(default_config_path, config_path)?;
                }

                // read the config.json
                let config_path = forg_dir_path.join("config.json");
                if !config_path.exists() {
                    panic!(
                        "Unknown error: config_path for {} undefined. run forg --init and try again.",
                        config_path.display()
                    )
                }
                let config_data_str = fs::read_to_string(&config_path)?;
                let config_json: Vec<ConfigItem> = serde_json::from_str(config_data_str.as_str())?;
                println!("config_json: {:?}", config_json);
            }
            None => println!("Home directory not found"),
        }
    } else if let Some(target_dir) = cli.exec {
        // check if the directory exists
        match env::home_dir() {
            Some(path) => {
                let mut forg_dir_path = PathBuf::from(&path);
                forg_dir_path.push(".forg");

                if !forg_dir_path.exists() {
                    panic!("unitility not initialised properly. run forg --init then try again.")
                }

                // read the config.json
                let config_path = forg_dir_path.join("config.json");
                if !config_path.exists() {
                    panic!(
                        "Unknown error: config_path for {} undefined. run forg --init and try again.",
                        config_path.display()
                    )
                }
                let config_data_str = fs::read_to_string(&config_path)?;
                let config_json: Vec<ConfigItem> = serde_json::from_str(config_data_str.as_str())?;
                println!("config_json: {:?}", config_json);

                // start organising files into designated paths
                println!("organising files into designated paths...");
                // pattern-directory map || pattern -> (pattern-regex, directory path)
                let mut directory_map: HashMap<String, (Regex, PathBuf)> = HashMap::new();
                for config_item in &config_json {
                    let folder_complete_path = PathBuf::from(path.join(&config_item.path));
                    let pattern = &config_item.pattern;
                    let pattern_regex = Regex::new(pattern).unwrap();
                    directory_map
                        .insert(String::from(pattern), (pattern_regex, folder_complete_path));
                }
                println!("directory_map: {:?}", directory_map);
                // check if the target folder exists and iterate files inside
                let target_path = PathBuf::from(path.join(target_dir));
                println!("target_folder {}", target_path.display());
                if !target_path.exists() {
                    panic!("No such directory found: {}", target_path.display())
                }
                let paths = fs::read_dir(target_path)?;
                for entry in paths {
                    let entry = entry?;
                    if entry.file_type()?.is_file() {
                        println!("path: {}", entry.path().display());
                        for (_, (pattern_regex, dir)) in &directory_map {
                            fs::create_dir_all(dir)?; // create the destination directory if needed
                            if let Some(filename_str) = entry.file_name().to_str() {
                                if pattern_regex.is_match(filename_str) {
                                    // move the file to destination directory
                                    let from_filepath = &entry.path();
                                    let to_filpath = dir.join(filename_str);
                                    println!(
                                        "moving from {} -> {}",
                                        from_filepath.display(),
                                        to_filpath.display()
                                    );
                                    fs::rename(from_filepath, to_filpath)?;
                                }
                            }
                        }
                    }
                }
            }
            None => panic!("utility no initialised. run forg --init then try again."),
        }
    } else {
        println!("Please provide an option. Use --help for info.")
    }

    Ok(())
}
