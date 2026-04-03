use std::fs;

use serde::{Deserialize, Serialize};

use crate::{kinds::phase::Phase, utils::path::state_path};

#[derive(Debug, Deserialize, Serialize)]
pub struct Store {
    timer_todo_id: Option<i32>,
    timer_cycle_phase: Phase,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            timer_todo_id: None,
            timer_cycle_phase: Phase::Work,
        }
    }
}

impl Store {
    pub fn load() -> Self {
        let path = state_path();
        fs::read_to_string(path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) {
        let path = state_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string(self) {
            let _ = fs::write(path, json);
        }
    }

    pub fn timer_todo_id(&self) -> Option<i32> {
        self.timer_todo_id
    }

    pub fn set_timer_todo_id(&mut self, todo_id: Option<i32>) -> &Self {
        self.timer_todo_id = todo_id;

        self
    }

    pub fn timer_cycle_phase(&self) -> &Phase {
        &self.timer_cycle_phase
    }

    pub fn set_timer_cycle_phase(&mut self, cycle_phase: Phase) -> &Self {
        self.timer_cycle_phase = cycle_phase;

        self
    }
}
