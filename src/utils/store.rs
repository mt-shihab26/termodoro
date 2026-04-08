use std::{collections::HashMap, fs};

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{kinds::phase::Phase, utils::path::store_path};

/// Persisted runtime state, loaded from and saved to disk on change.
#[derive(Debug, Deserialize, Serialize)]
pub struct Store {
    /// The currently selected todo id.
    timer_todo_id: Option<i32>,
    /// The current pomodoro cycle phase.
    timer_cycle_phase: Phase,
    /// Remaining milliseconds per todo, keyed by todo id (or `"none"` when no todo is selected).
    #[serde(default)]
    timer_remaining_millis: HashMap<String, u32>,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            timer_todo_id: None,
            timer_cycle_phase: Phase::Work,
            timer_remaining_millis: HashMap::new(),
        }
    }
}

fn todo_key(todo_id: Option<i32>) -> String {
    match todo_id {
        Some(id) => id.to_string(),
        None => "none".to_string(),
    }
}

impl Store {
    /// Loads the store from disk, returning the default if the file does not exist.
    pub fn load() -> Self {
        let path = store_path();
        fs::read_to_string(path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    /// Saves the store to disk.
    pub fn save(&self) {
        let path = store_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string(self) {
            let _ = fs::write(path, json);
        }
    }

    /// Returns the persisted todo id.
    pub fn timer_todo_id(&self) -> Option<i32> {
        self.timer_todo_id
    }

    /// Sets the todo id and returns `&Self` for chaining.
    pub fn set_timer_todo_id(&mut self, todo_id: Option<i32>) -> &Self {
        self.timer_todo_id = todo_id;
        self
    }

    /// Returns the persisted cycle phase.
    pub fn timer_cycle_phase(&self) -> &Phase {
        &self.timer_cycle_phase
    }

    /// Sets the cycle phase and returns `&Self` for chaining.
    pub fn set_timer_cycle_phase(&mut self, cycle_phase: Phase) -> &Self {
        self.timer_cycle_phase = cycle_phase;
        self
    }

    /// Returns the persisted remaining milliseconds for the given todo, if any.
    pub fn timer_remaining_for_todo(&self, todo_id: Option<i32>) -> Option<u32> {
        self.timer_remaining_millis.get(&todo_key(todo_id)).copied()
    }

    /// Sets the remaining milliseconds for the given todo and returns `&Self` for chaining.
    pub fn set_timer_remaining_for_todo(&mut self, todo_id: Option<i32>, millis: u32) -> &Self {
        self.timer_remaining_millis.insert(todo_key(todo_id), millis);
        self
    }

    /// Removes the persisted remaining milliseconds for the given todo and returns `&Self` for chaining.
    pub fn clear_timer_remaining_for_todo(&mut self, todo_id: Option<i32>) -> &Self {
        self.timer_remaining_millis.remove(&todo_key(todo_id));
        self
    }
}
