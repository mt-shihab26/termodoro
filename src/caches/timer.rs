use sea_orm::DatabaseConnection;

use crate::{
    kinds::page::Page,
    models::{session::Session, todo::Todo},
};

/// Session statistics for a single todo.
#[derive(Clone)]
pub struct Stat {
    /// Number of completed pomodoro sessions.
    pub sessions: u32,
    /// Total time spent in seconds across all sessions.
    pub secs: u32,
}

/// Per-tab cache for today's todos and their session stats.
pub struct TimerCache {
    db: DatabaseConnection,
    /// Cached list of today's todos, `None` until first fetch.
    todos: Option<Vec<Todo>>,
    /// Cached stats parallel to `todos`, `None` until first fetch.
    stats: Option<Vec<Stat>>,
}

impl TimerCache {
    /// Creates a new empty cache backed by the given database connection.
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db,
            todos: None,
            stats: None,
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

    /// Returns stats for all cached todos, querying the DB if needed.
    pub fn get_stats(&mut self) -> &[Stat] {
        if self.stats.is_none() {
            let ids: Vec<Option<i32>> = self.get_todos().iter().map(|t| t.id).collect();
            let stats = ids
                .into_iter()
                .map(|id| {
                    let (sessions, secs) = id.map(|id| Session::stats_for_todo(&self.db, id)).unwrap_or((0, 0));
                    Stat { sessions, secs }
                })
                .collect();
            self.stats = Some(stats);
        }
        self.stats.as_deref().unwrap_or(&[])
    }

    /// Returns the cached session stats for the given todo id, querying the DB if needed.
    pub fn get_stat(&mut self, todo_id: i32) -> Option<&Stat> {
        self.get_stats();
        let idx = self.todos.as_deref()?.iter().position(|t| t.id == Some(todo_id))?;
        self.stats.as_deref()?.get(idx)
    }

    /// Drops the cached todo list so the next call to `get_todos()` re-queries.
    pub fn invalidate_todos(&mut self) {
        self.todos = None;
        self.stats = None;
    }

    /// Drops the cached session stats so they are re-fetched on next render.
    pub fn invalidate_stats(&mut self) {
        self.stats = None;
    }
}
