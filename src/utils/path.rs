use std::env;
use std::path::PathBuf;

// TODO: get a separate function for getting local state path

pub fn log_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());

    PathBuf::from(home)
        .join(".local")
        .join("state")
        .join(env!("CARGO_PKG_NAME"))
        .join("orivo.log")
}

pub fn db_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home)
        .join(".local")
        .join("state")
        .join(env!("CARGO_PKG_NAME"))
        .join("orivo.db")
}
