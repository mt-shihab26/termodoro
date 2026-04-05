use std::{env, path::PathBuf};

/// Returns the path to the config file.
pub fn config_toml_path() -> PathBuf {
    #[cfg(debug_assertions)]
    return local().join("config.toml");

    #[cfg(not(debug_assertions))]
    config_path().join("config.toml")
}

/// Returns the path to the log file.
pub fn log_path() -> PathBuf {
    #[cfg(debug_assertions)]
    return local().join("orivo.log");

    #[cfg(not(debug_assertions))]
    local_path().join("orivo.log")
}

/// Returns the path to the database file.
pub fn db_path() -> PathBuf {
    #[cfg(debug_assertions)]
    return local().join("orivo.sqlite");

    #[cfg(not(debug_assertions))]
    local_path().join("orivo.sqlite")
}

/// Returns the path to the persisted timer state file.
pub fn store_path() -> PathBuf {
    #[cfg(debug_assertions)]
    return local().join("store.json");

    #[cfg(not(debug_assertions))]
    local_path().join("store.json")
}

/// the base directory for runtime state files
#[cfg_attr(debug_assertions, allow(dead_code))]
fn local_path() -> PathBuf {
    PathBuf::from(home())
        .join(".local")
        .join("state")
        .join(env!("CARGO_PKG_NAME"))
}

/// The base directory for config files.
#[cfg_attr(debug_assertions, allow(dead_code))]
fn config_path() -> PathBuf {
    PathBuf::from(home()).join(".config").join(env!("CARGO_PKG_NAME"))
}

/// Returns the user's home directory, handling both Unix (`HOME`) and Windows (`USERPROFILE`).
fn home() -> String {
    env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string())
}

/// Returns the local dev directory used for all files in debug builds.
fn local() -> PathBuf {
    PathBuf::from("./local")
}
