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

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Config {
    pub turso: Option<TursoConfig>,
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
