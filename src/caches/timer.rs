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
            stat: None,
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

    /// Returns the cached session stats for the selected todo, querying the DB if needed.
    pub fn get_stat(&mut self, todo_id: i32) -> Option<&Stat> {
        if self.stat.is_none() {
            let (sessions, secs) = Session::stats_for_todo(&self.db, todo_id);
            self.stat = Some(Stat { sessions, secs });
        }
        self.stat.as_ref()
    }

    /// Drops the cached todo list so the next call to `get_todos()` re-queries.
    pub fn invalidate_todos(&mut self) {
        self.todos = None;
    }

    /// Drops the cached session stats so they are re-fetched on next render.
    pub fn invalidate_stats(&mut self) {
        self.stat = None;
    }
}
