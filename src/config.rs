use serde::Deserialize;
use std::fs::read_to_string;
use std::path::PathBuf;

use crate::logger::log;

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
        log("config: could not resolve config path (HOME not set?), using defaults");
        return Config::default();
    };

    let Ok(contents) = read_to_string(&path) else {
        log(&format!("config: could not read {}, using defaults", path.display()));
        return Config::default();
    };

    let Ok(config) = serde_json::from_str(&contents) else {
        log(&format!("config: failed to parse {}, using defaults", path.display()));
        return Config::default();
    };

    config
}

fn config_path() -> Option<PathBuf> {
    let base = match std::env::var("XDG_CONFIG_HOME") {
        Ok(xdg) => PathBuf::from(xdg),
        Err(e) => {
            log(&format!("config: XDG_CONFIG_HOME not set ({e}), falling back to HOME"));
            match std::env::var("HOME") {
                Ok(home) => PathBuf::from(home).join(".config"),
                Err(e) => {
                    log(&format!("config: HOME env var not set: {e}"));
                    return None;
                }
            }
        }
    };

    let path = base.join("termodoro").join("config.json");

    Some(path)
}
