use log::warn;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

/// Returns the default config file path: ~/.config/rugby-tray/config.toml
pub fn default_config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("/home/user/.config")) // fallback if config_dir is not available
        .join("prop_time")
        .join("config.toml")
}

/// Top-level user configuration structure
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub version: u8,
    pub teams: Vec<String>,
    pub match_count: usize,
    pub refresh: RefreshInterval,
    pub theme: Theme,
    pub formatting: Formatting,
}

/// Refresh interval can be "minutes:N" or "boot-only"
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RefreshInterval {
    Minutes(u64),
    BootOnly,
}

impl Default for RefreshInterval {
    fn default() -> Self {
        RefreshInterval::Minutes(60)
    }
}

/// Light or Dark theme for formatting
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Dark
    }
}

/// Formatting options for date, venue, etc.
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Formatting {
    pub datetime_format: String,
    pub show_venue: bool,
    pub show_competition: bool,
}

impl Default for Formatting {
    fn default() -> Self {
        Self {
            datetime_format: "%a %e %b, %H:%M".to_string(),
            show_venue: false,
            show_competition: false,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: 1,
            teams: vec!["leinster".to_string()],
            match_count: 3,
            refresh: RefreshInterval::Minutes(60),
            theme: Theme::Dark,
            formatting: Formatting::default(),
        }
    }
}

/// Loads the config from disk, or falls back to default if file is missing or invalid.
impl Config {
    pub fn load_config() -> Self {
        let path = default_config_path();
        match fs::read_to_string(&path) {
            Ok(contents) => match toml::from_str(&contents) {
                Ok(cfg) => cfg,
                Err(err) => {
                    warn!("Failed to parse config file: {err}. Falling back to defaults.");
                    Config::default()
                }
            },
            Err(err) => {
                warn!("Could not read config file: {err}. Falling back to defaults.");
                Config::default()
            }
        }
    }
}
