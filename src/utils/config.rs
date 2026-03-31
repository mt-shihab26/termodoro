use std::env;
use std::fs;
use std::io::{Error, ErrorKind, Result};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TursoConfig {
    pub url: String,
    pub token: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TimerConfig {
    #[serde(default = "TimerConfig::default_show_millis")]
    pub show_millis: bool,
    #[serde(default = "TimerConfig::default_work_duration")]
    pub work_duration: u64,
    #[serde(default = "TimerConfig::default_break_duration")]
    pub break_duration: u64,
    #[serde(default = "TimerConfig::default_long_break_duration")]
    pub long_break_duration: u64,
    #[serde(default = "TimerConfig::default_long_break_interval")]
    pub long_break_interval: u64,
}

impl TimerConfig {
    fn default_show_millis() -> bool {
        true
    }
    fn default_work_duration() -> u64 {
        25
    }
    fn default_break_duration() -> u64 {
        5
    }
    fn default_long_break_duration() -> u64 {
        15
    }
    fn default_long_break_interval() -> u64 {
        4
    }
}

impl Default for TimerConfig {
    fn default() -> Self {
        Self {
            show_millis: Self::default_show_millis(),
            work_duration: Self::default_work_duration(),
            break_duration: Self::default_break_duration(),
            long_break_duration: Self::default_long_break_duration(),
            long_break_interval: Self::default_long_break_interval(),
        }
    }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Config {
    pub turso: Option<TursoConfig>,
    #[serde(default)]
    pub timer: TimerConfig,
}

impl Config {
    pub fn path() -> PathBuf {
        let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(".config").join("orivo").join("config.toml")
    }

    pub fn load() -> Result<Self> {
        let path = Self::path();
        if !path.exists() {
            return Ok(Self::default());
        }
        let raw = fs::read_to_string(&path)?;
        toml::from_str(&raw).map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))
    }

    /// Creates a template config file if one does not already exist.
    /// Returns the path where it was written.
    pub fn create_template() -> Result<PathBuf> {
        let path = Self::path();
        if path.exists() {
            return Ok(path);
        }
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(
            &path,
            "# Orivo configuration\n\
             #\n\
             # Get your credentials:\n\
             #   turso auth login\n\
             #   turso db create orivo\n\
             #   turso db show orivo --url\n\
             #   turso db tokens create orivo\n\
             \n\
             [turso]\n\
             url   = \"libsql://your-db-name.turso.io\"\n\
             token = \"your-auth-token\"\n",
        )?;
        Ok(path)
    }
}
