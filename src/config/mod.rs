pub mod db;
pub mod timer;

use std::env;
use std::fs;
use std::io::{Error, ErrorKind, Result};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::config::db::DBConfig;
use crate::config::timer::TimerConfig;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub show_fps: bool,
    #[serde(default)]
    pub db: DBConfig,
    #[serde(default)]
    pub timer: TimerConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            show_fps: false,
            db: Default::default(),
            timer: Default::default(),
        }
    }
}

impl Config {
    pub fn path() -> PathBuf {
        PathBuf::from(env::var("HOME").unwrap_or_else(|_| ".".to_string()))
            .join(".config")
            .join("orivo")
            .join("config.toml")
    }

    pub fn load() -> Result<Self> {
        let path = Self::path();
        if !path.exists() {
            return Ok(Self::default());
        }
        let raw = fs::read_to_string(&path)?;
        toml::from_str(&raw).map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))
    }
}
