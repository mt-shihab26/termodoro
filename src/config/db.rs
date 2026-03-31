use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DBConfig {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub token: String,
}

impl Default for DBConfig {
    fn default() -> Self {
        Self {
            url: String::new(),
            token: String::new(),
        }
    }
}

impl DBConfig {
    pub fn is_configured(&self) -> bool {
        !self.url.is_empty() && !self.token.is_empty()
    }
}
