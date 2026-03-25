use serde::Deserialize;
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Deserialize)]
#[serde(default)]
pub struct Config {
    pub work_session_duration: u64,
    pub break_session_duration: u64,
    pub long_break_session_duration: u64,
    pub long_break_session_interval: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            work_session_duration: 25,
            break_session_duration: 5,
            long_break_session_duration: 15,
            long_break_session_interval: 4,
        }
    }
}

pub fn load_config() -> Config {
    let Some(path) = config_path() else {
        return Config::default();
    };

    let Ok(contents) = read_to_string(path) else {
        return Config::default();
    };

    let Ok(config) = serde_json::from_str(&contents) else {
        return Config::default();
    };

    config
}

fn config_path() -> Option<PathBuf> {
    let base = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(std::env::var("HOME").unwrap_or_default()).join(".config")
        });

    Some(base.join("termodoro").join("config.json"))
}
