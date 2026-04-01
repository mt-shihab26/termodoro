use sea_orm::DatabaseConnection;

use crate::kinds::page::Page;
use crate::models::{session::Session, todo::Todo};

pub struct TimerCache {
    db: DatabaseConnection,
    todos: Option<Vec<Todo>>,
    stats: (u32, Option<(u32, u32)>),
}

impl TimerCache {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db,
            todos: None,
            stats: (u32::MAX, None),
        }
    }

    /// Returns the cached todo list, querying the DB if needed.
    pub fn todos(&mut self) -> &[Todo] {
        if self.todos.is_none() {
            self.todos = Some(Todo::list(&self.db, Page::Today, 0, 100));
        }
        self.todos.as_deref().unwrap_or(&[])
    }

    /// Drops the cached todo list so the next call to `todos()` re-queries.
    pub fn invalidate_todos(&mut self) {
        self.todos = None;
    }

    /// Drops the cached session stats so they are re-fetched on next render.
    pub fn invalidate_stats(&mut self) {
        self.stats = (u32::MAX, None);
    }

    /// Re-fetches session stats when `sessions` counter has changed or cache is empty.
    pub fn refresh_stats_if_needed(&mut self, todo_id: i32, sessions: u32) {
        if self.stats.1.is_none() || self.stats.0 != sessions {
            self.stats.1 = Some(Session::stats_for_todo(&self.db, todo_id));
            self.stats.0 = sessions;
        }
    }

    /// Returns the cached todo with the given id, querying the DB if needed.
    pub fn get_todo(&mut self, id: i32) -> Option<&Todo> {
        self.todos();
        self.todos.as_deref()?.iter().find(|t| t.id == Some(id))
    }

    pub fn get_stats(&self) -> Option<(u32, u32)> {
        self.stats.1
    }
}
