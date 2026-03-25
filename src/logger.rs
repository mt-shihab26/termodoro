use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn log(msg: &str) {
    let Some(path) = log_path() else {
        return;
    };

    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let line = format!("[{timestamp}] {msg}\n");

    let file = OpenOptions::new().create(true).append(true).open(path);

    match file {
        Ok(mut f) => {
            let _ = f.write_all(line.as_bytes());
        }
        Err(e) => {
            eprintln!("logger: failed to open log file: {e}");
        }
    }
}

fn log_path() -> Option<PathBuf> {
    let state_home = std::env::var("XDG_STATE_HOME");

    let base;
    if state_home.is_ok() {
        base = PathBuf::from(state_home.unwrap());
    } else {
        let home = std::env::var("HOME").ok()?;
        base = PathBuf::from(home).join(".local").join("state");
    }

    Some(base.join("termodoro").join("termodoro.log"))
}
