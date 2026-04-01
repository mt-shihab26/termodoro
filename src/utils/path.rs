use std::env;
use std::path::PathBuf;

pub fn log_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());

    PathBuf::from(home)
        .join(".local/state")
        .join(env!("CARGO_PKG_NAME"))
        .join("orivo.log")
}

pub fn db_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home)
        .join(".local")
        .join("state")
        .join("orivo")
        .join("orivo.db")
}
