use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_work")]
    pub work_session_duration: u64,

    #[serde(default = "default_break")]
    pub break_session_duration: u64,

    #[serde(default = "default_long_break")]
    pub long_break_session_duration: u64,

    #[serde(default = "default_interval")]
    pub long_break_session_interval: u32,
}

fn default_work() -> u64 { 25 }
fn default_break() -> u64 { 5 }
fn default_long_break() -> u64 { 15 }
fn default_interval() -> u32 { 4 }

impl Default for Config {
    fn default() -> Self {
        Self {
            work_session_duration: default_work(),
            break_session_duration: default_break(),
            long_break_session_duration: default_long_break(),
            long_break_session_interval: default_interval(),
        }
    }
}

pub fn load_config() -> Config {
    config_path()
        .and_then(|p| fs::read_to_string(p).ok())
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn config_path() -> Option<PathBuf> {
    let base = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").expect("HOME not set");
            PathBuf::from(home).join(".config")
        });
    Some(base.join("termodoro").join("config.json"))
}
