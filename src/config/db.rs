use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DBConfig {
    pub url: String,
    pub token: String,
}
