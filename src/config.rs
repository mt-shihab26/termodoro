use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Debug, Clone)]
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
    let mut config = Config::default();

    let Some(path) = config_path() else {
        return config;
    };

    let Ok(contents) = read_to_string(path) else {
        return config;
    };

    let map = parse_json_object(&contents);

    if let Some(v) = map
        .get("work_session_duration")
        .and_then(|s| s.parse().ok())
    {
        config.work_session_duration = v;
    }
    if let Some(v) = map
        .get("break_session_duration")
        .and_then(|s| s.parse().ok())
    {
        config.break_session_duration = v;
    }
    if let Some(v) = map
        .get("long_break_session_duration")
        .and_then(|s| s.parse().ok())
    {
        config.long_break_session_duration = v;
    }
    if let Some(v) = map
        .get("long_break_session_interval")
        .and_then(|s| s.parse().ok())
    {
        config.long_break_session_interval = v;
    }

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

/// Parses a flat JSON object of string keys and number values.
/// Returns a map of key -> raw value string.
fn parse_json_object(s: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    // Strip outer braces
    let inner = s.trim().trim_start_matches('{').trim_end_matches('}');
    for entry in inner.split(',') {
        let entry = entry.trim();
        if entry.is_empty() {
            continue;
        }
        // Split on first ':'
        let Some(colon) = entry.find(':') else {
            continue;
        };
        let key = entry[..colon].trim().trim_matches('"').to_string();
        let value = entry[colon + 1..].trim().trim_matches('"').to_string();
        if !key.is_empty() && !value.is_empty() {
            map.insert(key, value);
        }
    }
    map
}
