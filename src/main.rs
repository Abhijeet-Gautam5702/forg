use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env, fs,
    io::Result,
    path::{Path, PathBuf},
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
    exec: String,
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
    } else {
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
                // pattern-directory map
                let mut directory_map: HashMap<String, PathBuf> = HashMap::new();
                for config_item in &config_json {
                    let folder_complete_path = PathBuf::from(path.join(&config_item.path));
                    let pattern = &config_item.pattern;
                    directory_map.insert(String::from(pattern), folder_complete_path);
                }
                println!("directory_map: {:?}", directory_map);
                // check if the target folder exists and iterate files inside
                let target_path = PathBuf::from(path.join(cli.exec));
                println!("target_folder {}", target_path.display());
                if !target_path.exists() {
                    panic!("No such directory found: {}", target_path.display())
                }
            }
            None => panic!("utility no initialised. run forg --init then try again."),
        }
    }

    Ok(())
}
