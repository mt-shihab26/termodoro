use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn log(msg: &str) {
    let Some(path) = log_path() else {
        eprintln!("logger: could not resolve log path (HOME not set?)");
        return;
    };

    if let Some(dir) = path.parent() {
        if let Err(e) = std::fs::create_dir_all(dir) {
            eprintln!("logger: could not create log directory {}: {e}", dir.display());
            return;
        }
    }

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let line = format!("[{timestamp}] {msg}\n");

    match OpenOptions::new().create(true).append(true).open(&path) {
        Ok(mut f) => {
            if let Err(e) = f.write_all(line.as_bytes()) {
                eprintln!("logger: failed to write to {}: {e}", path.display());
            }
        }
        Err(e) => {
            eprintln!("logger: failed to open {}: {e}", path.display());
        }
    }
}

fn log_path() -> Option<PathBuf> {
    let base = match std::env::var("XDG_STATE_HOME") {
        Ok(state_home) => PathBuf::from(state_home),
        Err(e) => {
            eprintln!("logger: XDG_STATE_HOME not set ({e}), falling back to HOME");
            match std::env::var("HOME") {
                Ok(home) => PathBuf::from(home).join(".local").join("state"),
                Err(e) => {
                    eprintln!("logger: HOME env var not set: {e}");
                    return None;
                }
            }
        }
    };

    Some(base.join("termodoro").join("termodoro.log"))
}
