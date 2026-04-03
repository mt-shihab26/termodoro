use std::env;
use std::path::PathBuf;

fn local() -> PathBuf {
    PathBuf::from("./local")
}

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
    local_state_path().join("orivo.log")
}

/// Returns the path to the database file.
pub fn db_path() -> PathBuf {
    #[cfg(debug_assertions)]
    return local().join("orivo.sqlite");

    #[cfg(not(debug_assertions))]
    local_state_path().join("orivo.sqlite")
}

/// the base directory for runtime state files
#[cfg_attr(debug_assertions, allow(dead_code))]
fn local_state_path() -> PathBuf {
    PathBuf::from(home())
        .join(".local")
        .join("state")
        .join(env!("CARGO_PKG_NAME"))
}

///  the base directory for initail config files
#[cfg_attr(debug_assertions, allow(dead_code))]
fn config_path() -> PathBuf {
    PathBuf::from(home())
        .join(".config")
        .join(env!("CARGO_PKG_NAME"))
        .join("config.toml")
}

/// Returns the user's home directory
fn home() -> String {
    env::var("HOME").unwrap_or_else(|_| ".".to_string())
}
