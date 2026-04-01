use std::env;
use std::path::PathBuf;

fn local_state_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());

    PathBuf::from(home)
        .join(".local")
        .join("state")
        .join(env!("CARGO_PKG_NAME"))
}

pub fn log_path() -> PathBuf {
    local_state_path().join("orivo.log")
}

pub fn db_path() -> PathBuf {
    local_state_path().join("orivo.db")
}
