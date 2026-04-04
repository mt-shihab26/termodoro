//! Database configuration values loaded from the user config file.

use serde::{Deserialize, Serialize};

/// Database connection configuration, loaded from the user's config file.
#[derive(Debug, Deserialize, Serialize)]
pub struct DBConfig {
    /// Remote database URL.
    #[serde(default)]
    pub url: String,
    /// Authentication token for the remote database.
    #[serde(default)]
    pub token: String,
}

impl Default for DBConfig {
    /// Returns an empty database configuration.
    fn default() -> Self {
        Self {
            url: String::new(),
            token: String::new(),
        }
    }
}

impl DBConfig {
    /// Returns `true` if both `url` and `token` are non-empty.
    pub fn is_configured(&self) -> bool {
        !self.url.is_empty() && !self.token.is_empty()
    }
}
