use std::{fs, path::PathBuf};

use orivo::{log_error, log_info, log_warn};

fn log_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home)
        .join(".local/state")
        .join(env!("CARGO_PKG_NAME"))
        .join("orivo.log")
}

fn read_log() -> String {
    fs::read_to_string(log_path()).unwrap_or_default()
}

#[test]
fn test_log_error_writes_to_file() {
    log_error!("test error message");
    assert!(read_log().contains("ERROR: test error message"));
}

#[test]
fn test_log_warn_writes_to_file() {
    log_warn!("test warn message");
    assert!(read_log().contains("WARN: test warn message"));
}

#[test]
fn test_log_info_writes_to_file() {
    log_info!("test info message");
    assert!(read_log().contains("INFO: test info message"));
}

#[test]
fn test_log_format_has_timestamp() {
    log_error!("timestamp format check");
    // expect somewhere in the file: [YYYY-MM-DDTHH:MM:SSZ] ERROR: timestamp format check
    let contents = read_log();
    assert!(
        contents.contains("ERROR: timestamp format check"),
        "entry missing"
    );
    assert!(contents.contains('T'), "timestamp missing T separator");
    assert!(contents.contains('Z'), "timestamp missing Z suffix");
    assert!(contents.contains('['), "timestamp missing opening bracket");
}

#[test]
fn test_log_appends_not_overwrites() {
    log_error!("first entry");
    log_error!("second entry");
    let contents = read_log();
    assert!(contents.contains("first entry"));
    assert!(contents.contains("second entry"));
}

#[test]
fn test_log_format_args() {
    let code = 42;
    let msg = "something broke";
    log_error!("error code {code}: {msg}");
    assert!(read_log().contains("error code 42: something broke"));
}
