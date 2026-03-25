use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::read_to_string;
use std::path::PathBuf;

use crate::logger::log;
use crate::timer::{Phase, Timer};

#[derive(Serialize, Deserialize)]
pub struct State {
    pub phase: Phase,
    pub remaining_secs: u64,
    pub sessions_completed: u32,
}

pub fn load_state() -> Option<State> {
    let Some(path) = state_path() else {
        log("state: could not resolve state path (HOME not set?)");
        return None;
    };

    let Ok(contents) = read_to_string(&path) else {
        log(&format!("state: could not read {}", path.display()));
        return None;
    };

    let Ok(state) = serde_json::from_str(&contents) else {
        log(&format!("state: failed to parse {}", path.display()));
        return None;
    };

    Some(state)
}

pub fn save_state(timer: &Timer) {
    let Some(path) = state_path() else {
        log("state: could not resolve state path (HOME not set?)");
        return;
    };

    if let Some(dir) = path.parent() {
        if let Err(e) = fs::create_dir_all(dir) {
            log(&format!("state: could not create directory {}: {e}", dir.display()));
            return;
        }
    }

    let state = State {
        phase: timer.phase.clone(),
        remaining_secs: timer.remaining_secs,
        sessions_completed: timer.sessions_completed,
    };

    let Ok(contents) = serde_json::to_string(&state) else {
        log("state: failed to serialize state");
        return;
    };

    if let Err(e) = fs::write(&path, contents) {
        log(&format!("state: failed to write {}: {e}", path.display()));
    }
}

fn state_path() -> Option<PathBuf> {
    let base = match std::env::var("XDG_STATE_HOME") {
        Ok(state_home) => PathBuf::from(state_home),
        Err(e) => {
            log(&format!("state: XDG_STATE_HOME not set ({e}), falling back to HOME"));
            match std::env::var("HOME") {
                Ok(home) => PathBuf::from(home).join(".local").join("state"),
                Err(e) => {
                    log(&format!("state: HOME env var not set: {e}"));
                    return None;
                }
            }
        }
    };

    Some(base.join("termodoro").join("state.json"))
}
