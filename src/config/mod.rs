/// Database connection configuration loaded from `config.toml`.
pub mod db;
/// Pomodoro timer configuration loaded from `config.toml`.
pub mod timer;

use std::{
    fs,
    io::{Error, ErrorKind, Result},
};

use serde::{Deserialize, Serialize};

use crate::{
    config::{db::DBConfig, timer::TimerConfig},
    utils::path::config_path,
};

/// Top-level application configuration, loaded from `~/.config/orivo/config.toml`.
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /// Whether to show the FPS counter in the UI.
    #[serde(default)]
    pub show_fps: bool,
    /// Database connection settings.
    #[serde(default)]
    pub db: DBConfig,
    /// Pomodoro timer settings.
    #[serde(default)]
    pub timer: TimerConfig,
}

impl Default for Config {
    /// Returns the built-in default configuration values.
    fn default() -> Self {
        Self {
            show_fps: false,
            db: Default::default(),
            timer: Default::default(),
        }
    }
}

impl Config {
    /// Loads the config from disk, returning the default if the file does not exist.
    pub fn load() -> Result<Self> {
        let path = config_path();
        if !path.exists() {
            return Ok(Self::default());
        }
        let raw = fs::read_to_string(&path)?;
        toml::from_str(&raw).map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))
    }
}
