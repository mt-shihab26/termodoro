use std::fs;
use std::path::PathBuf;

use crate::timer::{Phase, Timer};

pub struct State {
    pub phase: Phase,
    pub remaining_secs: u64,
    pub sessions_completed: u32,
}

pub fn save(timer: &Timer) {
    let Some(path) = state_path() else { return };
    if let Some(dir) = path.parent() {
        let _ = fs::create_dir_all(dir);
    }
    let contents = format!(
        "phase={}\nremaining_secs={}\nsessions_completed={}\n",
        phase_to_str(&timer.phase),
        timer.remaining_secs,
        timer.sessions_completed,
    );
    let _ = fs::write(path, contents);
}

pub fn load_state() -> Option<State> {
    let contents = fs::read_to_string(state_path()?).ok()?;
    let mut phase = None;
    let mut remaining_secs = None;
    let mut sessions_completed = None;

    for line in contents.lines() {
        let Some(eq) = line.find('=') else { continue };
        let key = &line[..eq];
        let val = &line[eq + 1..];
        match key {
            "phase" => phase = str_to_phase(val),
            "remaining_secs" => remaining_secs = val.parse().ok(),
            "sessions_completed" => sessions_completed = val.parse().ok(),
            _ => {}
        }
    }

    Some(State {
        phase: phase?,
        remaining_secs: remaining_secs?,
        sessions_completed: sessions_completed?,
    })
}

fn phase_to_str(phase: &Phase) -> &'static str {
    match phase {
        Phase::Work => "work",
        Phase::Break => "break",
        Phase::LongBreak => "long_break",
    }
}

fn str_to_phase(s: &str) -> Option<Phase> {
    match s {
        "work" => Some(Phase::Work),
        "break" => Some(Phase::Break),
        "long_break" => Some(Phase::LongBreak),
        _ => None,
    }
}

fn state_path() -> Option<PathBuf> {
    let base = std::env::var("XDG_STATE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(std::env::var("HOME").unwrap_or_default()).join(".local/state"));
    Some(base.join("termodoro").join("state"))
}
