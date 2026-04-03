use std::fs;

use serde::{Deserialize, Serialize};

use crate::utils::path::state_path;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct State {
    pub todo_id: Option<i32>,
}

impl State {
    pub fn new(todo_id: Option<i32>) -> Self {
        Self { todo_id }
    }

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
}
