use std::env;
use std::path::PathBuf;

/// Returns the user's home directory, falling back to `"."` if `$HOME` is unset.
fn home() -> String {
    env::var("HOME").unwrap_or_else(|_| ".".to_string())
}

/// Returns `~/.local/state/<pkg>` — the base directory for runtime state files.
fn local_state_path() -> PathBuf {
    PathBuf::from(home())
        .join(".local")
        .join("state")
        .join(env!("CARGO_PKG_NAME"))
}

/// Returns the path to the config file (`~/.config/<pkg>/config.toml`).
pub fn config_path() -> PathBuf {
    PathBuf::from(home())
        .join(".config")
        .join(env!("CARGO_PKG_NAME"))
        .join("config.toml")
}

/// Returns the path to the log file (`~/.local/state/<pkg>/orivo.log`).
pub fn log_path() -> PathBuf {
    local_state_path().join("orivo.log")
}

/// Returns the path to the local database file (`~/.local/state/<pkg>/orivo.db`).
pub fn db_path() -> PathBuf {
    local_state_path().join("orivo.db")
}
