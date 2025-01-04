use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::Error;
use toml;
use xdg::BaseDirectories;

const APP: &str = "ask";
const DEFAULT_THEME: &str = "default";
const THEMES_DIR: &str = "themes";
const DEFAULT_THEME_CSS: &str = include_str!("../assets/default.css");

/// Represents the configuration for the application.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    theme: Option<String>,
    close_on_submit: Option<bool>,
    close_on_blur: Option<bool>,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let xdg_dirs = BaseDirectories::with_prefix(APP)
            .map_err(|_| ConfigError::XdgError("Could not find XDG directories".to_string()))?;

        let config_dir = xdg_dirs.get_config_home();
        let config_path = config_dir.join(format!("{}.toml", APP));

        let config = if config_path.exists() {
            let config_str =
                std::fs::read_to_string(&config_path).map_err(|e| ConfigError::IoError(e))?;
            toml::from_str(&config_str).map_err(|e| ConfigError::TomlError(e))?
        } else {
            Config::default()
        };

        let themes_dir = xdg_dirs
            .place_config_file(THEMES_DIR)
            .map_err(|_| ConfigError::XdgError("Could not find XDG data directory".to_string()))?;

        std::fs::create_dir_all(&themes_dir).map_err(|e| ConfigError::IoError(e))?;

        let theme_name = config.theme.as_deref().unwrap_or(DEFAULT_THEME);
        let theme_path = themes_dir.join(format!("{}.css", theme_name));

        if !theme_path.exists() {
            std::fs::write(&theme_path, DEFAULT_THEME_CSS).map_err(|e| ConfigError::IoError(e))?;
        }

        Ok(config)
    }

    pub fn load_css(&self) -> Result<String, String> {
        let xdg_dirs = BaseDirectories::with_prefix(APP)
            .map_err(|_| "Could not find XDG directories".to_string())?;
        let themes_dir = xdg_dirs
            .place_config_file(THEMES_DIR)
            .map_err(|_| "Could not find XDG data directory".to_string())?;
        let theme_name = self.theme.as_deref().unwrap_or(DEFAULT_THEME);
        let theme_path = themes_dir.join(format!("{}.css", theme_name));

        std::fs::read_to_string(&theme_path)
            .map_err(|e| format!("Failed to read theme file: {}", e))
    }
}

#[derive(Debug)]
pub enum ConfigError {
    IoError(Error),
    TomlError(toml::de::Error),
    XdgError(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::IoError(e) => write!(f, "IO Error: {}", e),
            ConfigError::TomlError(e) => write!(f, "TOML Error: {}", e),
            ConfigError::XdgError(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::IoError(e) => Some(e),
            ConfigError::TomlError(e) => Some(e),
            ConfigError::XdgError(_) => None,
        }
    }
}
