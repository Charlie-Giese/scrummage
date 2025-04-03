use config::{Config as ConfigLoader, File, FileFormat};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub preferences: Preferences,
    pub formatting: Formatting,
}

#[derive(Debug, Deserialize)]
pub struct Preferences {
    pub teams: Vec<String>,
    pub matches_to_display: usize,
}

#[derive(Debug, Deserialize)]
pub struct Formatting {
    pub icon_style: String,
    pub date_format: String,
}

impl Config {
    pub fn load() -> Self {
        let config_path = "config.toml";

        let builder = ConfigLoader::builder();

        let builder = if Path::new(config_path).exists() {
            builder.add_source(File::new(config_path, FileFormat::Toml))
        } else {
            println!("Config file not found, using defaults.");
            builder
        };

        builder
            .build()
            .unwrap()
            .try_deserialize()
            .expect("Failed to load config")
    }
}
