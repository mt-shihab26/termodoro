use sea_orm::DatabaseConnection;

use crate::kinds::page::Page;
use crate::models::{session::Session, todo::Todo};

pub struct Stat {
    pub sessions: u32,
    pub secs: u32,
}

pub struct TimerCache {
    db: DatabaseConnection,
    todos: Option<Vec<Todo>>,
    stats: (u32, Option<Stat>),
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
    pub fn get_todos(&mut self) -> &[Todo] {
        if self.todos.is_none() {
            self.todos = Some(Todo::list(&self.db, Page::Today, 0, 100));
        }
        self.todos.as_deref().unwrap_or(&[])
    }

    /// Returns the cached todo with the given id, querying the DB if needed.
    pub fn get_todo(&mut self, id: i32) -> Option<&Todo> {
        self.get_todos();
        self.todos.as_deref()?.iter().find(|t| t.id == Some(id))
    }

    /// Returns the cached session stats for the selected todo.
    pub fn get_stat(&self) -> Option<&Stat> {
        self.stats.1.as_ref()
    }

    // -------------------------------

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
            let (sessions_count, secs) = Session::stats_for_todo(&self.db, todo_id);
            self.stats.1 = Some(Stat {
                sessions: sessions_count,
                secs,
            });
            self.stats.0 = sessions;
        }
    }
}
