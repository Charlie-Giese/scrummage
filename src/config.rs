use config::{Config as ConfigLoader, ConfigError, File, FileFormat};
use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path};

use crate::TeamScope;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub preferences: Option<Preferences>,
    pub formatting: Option<Formatting>,
}

#[derive(Debug, Deserialize)]
pub struct Preferences {
    pub teams: Option<Vec<String>>,
    pub nfix: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct Formatting {
    pub icon_style: Option<String>,
    pub date_format: Option<String>,
}

impl AppConfig {
    pub fn try_load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = "config.toml";

        if !Path::new(config_path).exists() {
            return Err("Config file does not exist.".into());
        }

        let contents = fs::read_to_string(config_path)?;
        if contents.trim().is_empty() {
            return Err("Config file is empty.".into());
        }

        let config = ConfigLoader::builder()
            .add_source(File::new(config_path, FileFormat::Toml))
            .build()?;

        // Validate expected fields manually
        let teams = config
            .get::<Vec<String>>("preferences.teams")
            .map_err(|_| {
                "Missing or invalid field: preferences.teams (expected a list of strings)"
            })?;

        if teams.is_empty() {
            return Err("The 'preferences.teams' field must not be empty.".into());
        }

        config.get::<usize>("preferences.nfix").map_err(|_| {
            "Missing or invalid field: preferences.matches_to_display (expected an integer)"
        })?;

        config
            .get::<String>("formatting.icon_style")
            .map_err(|_| "Missing or invalid field: formatting.icon_style (expected a string)")?;

        config
            .get::<String>("formatting.date_format")
            .map_err(|_| "Missing or invalid field: formatting.date_format (expected a string)")?;

        // All required fields are present, so deserialize into AppConfig
        let final_config: AppConfig = config.try_deserialize()?;
        Ok(final_config)
    }
}
