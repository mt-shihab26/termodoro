use sea_orm::DatabaseConnection;

use crate::kinds::page::Page;
use crate::models::{session::Session, todo::Todo};

#[derive(Clone)]
pub struct Stat {
    pub sessions: u32,
    pub secs: u32,
}

pub struct TimerCache {
    db: DatabaseConnection,
    todos: Option<Vec<Todo>>,
    stats: Option<Vec<Stat>>,
}

impl TimerCache {
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
            let stats = self
                .get_todos()
                .iter()
                .map(|t| {
                    let (sessions, secs) = t.id.map(|id| Session::stats_for_todo(&self.db, id)).unwrap_or((0, 0));
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
