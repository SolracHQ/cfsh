use serde::{Deserialize, Serialize};

mod alias;
mod condition;
mod envar;
mod error;

pub use alias::Alias;
pub use envar::Envar;

use crate::log_and_exit;

use self::error::ParseError;

const CONFIG_DIR: &str = "cfsh";
const CONFIG_FILES: &[&str] = &["config.ron", "config.yaml", "config.json"];

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    aliases: Vec<Alias>,
    envars: Vec<Envar>,
}

impl Config {
    pub fn load() -> Result<Config, ParseError> {
        use std::fs::metadata;
        // Get config directory
        let config_dir = match directories::BaseDirs::new() {
            Some(dirs) => dirs.config_dir().join(CONFIG_DIR),
            None => {
                log_and_exit!("Could not find config directory");
            }
        };

        // Checl if config directory exists and create it if it doesn't
        if !metadata(&config_dir).map(|m| m.is_dir()).unwrap_or(false) {
            if let Err(err) = std::fs::create_dir_all(&config_dir) {
                log_and_exit!("Could not create config directory due to error: {}", err);
            }
        }

        // Check for .ron or .yaml or .json config files and load them
        // Using the first one found
        for file_name in CONFIG_FILES {
            let config_file = config_dir.join(file_name);
            if metadata(&config_file).map(|m| m.is_file()).unwrap_or(false) {
                let file = match std::fs::File::open(&config_file) {
                    Ok(file) => file,
                    Err(err) => log_and_exit!("Could not open config file due to error: {}", err),
                };
                return Ok(match *file_name {
                    "config.ron" => ron::de::from_reader(file)?,
                    "config.yaml" => serde_yaml::from_reader(file)?,
                    "config.json" => serde_json::from_reader(file)?,
                    _ => unreachable!("Unreachable due to CONFIG_FILES constant"),
                });
            }
        }
        log_and_exit!(
            "Could not find config file, please create one at {}",
            config_dir.display()
        );
    }

    pub fn aliases(&self) -> &[Alias] {
        &self.aliases
    }

    pub fn envars(&self) -> &[Envar] {
        &self.envars
    }
}
