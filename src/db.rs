use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{Connection, OptionalExtension, params};

use crate::logger::log;

#[derive(Debug, Clone)]
pub struct Project {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct TodoRow {
    pub id: i64,
    pub project_id: Option<i64>,
    pub title: String,
    pub due_date: Option<String>,
    pub completed_at: Option<i64>,
    pub work_secs: u64,
}

#[derive(Debug, Clone)]
pub struct TodoBrief {
    pub id: i64,
    pub title: String,
    pub project_name: Option<String>,
}

#[derive(Debug, Clone)]
pub enum TodoFilter {
    Index { show_completed: bool },
    Today { date: String, show_completed: bool },
    Project { project_id: i64, show_completed: bool },
}

pub struct Db {
    conn: Connection,
}

impl Db {
    pub fn open() -> Option<Self> {
        let Some(path) = db_path() else {
            log("db: could not resolve db path (HOME not set?)");
            return None;
        };

        if let Some(dir) = path.parent() {
            if let Err(e) = fs::create_dir_all(dir) {
                log(&format!("db: could not create directory {}: {e}", dir.display()));
                return None;
            }
        }

        let conn = match Connection::open(&path) {
            Ok(c) => c,
            Err(e) => {
                log(&format!("db: failed to open {}: {e}", path.display()));
                return None;
            }
        };

        if let Err(e) = conn.execute_batch("PRAGMA foreign_keys = ON;") {
            log(&format!("db: failed to enable foreign_keys: {e}"));
        }

        if let Err(e) = init_schema(&conn) {
            log(&format!("db: failed to init schema: {e}"));
            return None;
        }

        Some(Self { conn })
    }

    pub fn list_projects(&self) -> rusqlite::Result<Vec<Project>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name FROM projects WHERE archived = 0 ORDER BY name COLLATE NOCASE")?;
        let rows = stmt.query_map([], |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;
        rows.collect()
    }

    pub fn create_project(&self, name: &str) -> rusqlite::Result<i64> {
        let now = unix_now();
        self.conn.execute(
            "INSERT INTO projects (name, created_at, archived) VALUES (?1, ?2, 0)",
            params![name, now],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn create_todo(&self, project_id: Option<i64>, title: &str, due_date: Option<&str>) -> rusqlite::Result<i64> {
        let now = unix_now();
        self.conn.execute(
            "INSERT INTO todos (project_id, title, due_date, created_at, completed_at)
             VALUES (?1, ?2, ?3, ?4, NULL)",
            params![project_id, title, due_date, now],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn toggle_todo_completed(&self, todo_id: i64) -> rusqlite::Result<()> {
        let now = unix_now();
        let completed_at: Option<i64> = self
            .conn
            .query_row(
                "SELECT completed_at FROM todos WHERE id = ?1",
                params![todo_id],
                |row| row.get(0),
            )
            .optional()?;

        let new_value: Option<i64> = match completed_at {
            Some(_) => None,
            None => Some(now),
        };

        self.conn.execute(
            "UPDATE todos SET completed_at = ?1 WHERE id = ?2",
            params![new_value, todo_id],
        )?;

        Ok(())
    }

    pub fn set_todo_due_date(&self, todo_id: i64, due_date: Option<&str>) -> rusqlite::Result<()> {
        self.conn.execute(
            "UPDATE todos SET due_date = ?1 WHERE id = ?2",
            params![due_date, todo_id],
        )?;
        Ok(())
    }

    pub fn list_todos(&self, filter: TodoFilter) -> rusqlite::Result<Vec<TodoRow>> {
        let (where_sql, params_vec): (String, Vec<rusqlite::types::Value>) = match filter {
            TodoFilter::Index { show_completed } => {
                if show_completed {
                    ("1=1".to_string(), vec![])
                } else {
                    ("t.completed_at IS NULL".to_string(), vec![])
                }
            }
            TodoFilter::Today { date, show_completed } => {
                if show_completed {
                    ("t.due_date = ?1".to_string(), vec![rusqlite::types::Value::from(date)])
                } else {
                    (
                        "t.due_date = ?1 AND t.completed_at IS NULL".to_string(),
                        vec![rusqlite::types::Value::from(date)],
                    )
                }
            }
            TodoFilter::Project {
                project_id,
                show_completed,
            } => {
                if show_completed {
                    (
                        "t.project_id = ?1".to_string(),
                        vec![rusqlite::types::Value::from(project_id)],
                    )
                } else {
                    (
                        "t.project_id = ?1 AND t.completed_at IS NULL".to_string(),
                        vec![rusqlite::types::Value::from(project_id)],
                    )
                }
            }
        };

        let sql = format!(
            "SELECT
                t.id,
                t.project_id,
                t.title,
                t.due_date,
                t.completed_at,
                COALESCE(SUM(s.duration_secs), 0) AS work_secs
             FROM todos t
             LEFT JOIN pomodoro_sessions s
               ON s.todo_id = t.id AND s.phase = 'work'
             WHERE {where_sql}
             GROUP BY t.id
             ORDER BY
               (t.completed_at IS NOT NULL) ASC,
               (t.due_date IS NULL) ASC,
               t.due_date ASC,
               t.id DESC"
        );

        let mut stmt = self.conn.prepare(&sql)?;
        let rows = stmt.query_map(rusqlite::params_from_iter(params_vec.iter()), |row| {
            Ok(TodoRow {
                id: row.get(0)?,
                project_id: row.get(1)?,
                title: row.get(2)?,
                due_date: row.get(3)?,
                completed_at: row.get(4)?,
                work_secs: row.get::<_, i64>(5)?.max(0) as u64,
            })
        })?;
        rows.collect()
    }

    pub fn todo_brief(&self, todo_id: i64) -> rusqlite::Result<Option<TodoBrief>> {
        self.conn
            .query_row(
                "SELECT t.id, t.title, p.name
                 FROM todos t
                 LEFT JOIN projects p ON p.id = t.project_id
                 WHERE t.id = ?1",
                params![todo_id],
                |row| {
                    Ok(TodoBrief {
                        id: row.get(0)?,
                        title: row.get(1)?,
                        project_name: row.get(2)?,
                    })
                },
            )
            .optional()
    }

    pub fn insert_work_session(&self, todo_id: Option<i64>, duration_secs: u64, ended_at: i64) -> rusqlite::Result<()> {
        let started_at = ended_at - duration_secs as i64;
        self.conn.execute(
            "INSERT INTO pomodoro_sessions (todo_id, phase, started_at, ended_at, duration_secs)
             VALUES (?1, 'work', ?2, ?3, ?4)",
            params![todo_id, started_at, ended_at, duration_secs as i64],
        )?;
        Ok(())
    }

    pub fn get_active_todo_id(&self) -> rusqlite::Result<Option<i64>> {
        let v: Option<String> = self
            .conn
            .query_row("SELECT value FROM app_meta WHERE key = 'active_todo_id'", [], |row| {
                row.get(0)
            })
            .optional()?;

        Ok(v.and_then(|s| s.parse::<i64>().ok()))
    }

    pub fn set_active_todo_id(&self, todo_id: Option<i64>) -> rusqlite::Result<()> {
        match todo_id {
            Some(id) => {
                self.conn.execute(
                    "INSERT INTO app_meta (key, value) VALUES ('active_todo_id', ?1)
                     ON CONFLICT(key) DO UPDATE SET value = excluded.value",
                    params![id.to_string()],
                )?;
            }
            None => {
                self.conn
                    .execute("DELETE FROM app_meta WHERE key = 'active_todo_id'", [])?;
            }
        }
        Ok(())
    }
}

fn init_schema(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(
        "BEGIN;

         CREATE TABLE IF NOT EXISTS projects (
           id INTEGER PRIMARY KEY,
           name TEXT NOT NULL UNIQUE,
           created_at INTEGER NOT NULL,
           archived INTEGER NOT NULL DEFAULT 0
         );

         CREATE TABLE IF NOT EXISTS todos (
           id INTEGER PRIMARY KEY,
           project_id INTEGER NULL REFERENCES projects(id) ON DELETE SET NULL,
           title TEXT NOT NULL,
           due_date TEXT NULL,
           created_at INTEGER NOT NULL,
           completed_at INTEGER NULL
         );

         CREATE TABLE IF NOT EXISTS pomodoro_sessions (
           id INTEGER PRIMARY KEY,
           todo_id INTEGER NULL REFERENCES todos(id) ON DELETE SET NULL,
           phase TEXT NOT NULL,
           started_at INTEGER NOT NULL,
           ended_at INTEGER NOT NULL,
           duration_secs INTEGER NOT NULL
         );

         CREATE INDEX IF NOT EXISTS idx_todos_project_id ON todos(project_id);
         CREATE INDEX IF NOT EXISTS idx_todos_due_date ON todos(due_date);
         CREATE INDEX IF NOT EXISTS idx_sessions_todo_id ON pomodoro_sessions(todo_id);
         CREATE INDEX IF NOT EXISTS idx_sessions_ended_at ON pomodoro_sessions(ended_at);

         CREATE TABLE IF NOT EXISTS app_meta (
           key TEXT PRIMARY KEY,
           value TEXT NOT NULL
         );

         COMMIT;",
    )?;
    Ok(())
}

fn db_path() -> Option<PathBuf> {
    let base = match std::env::var("XDG_DATA_HOME") {
        Ok(data_home) => PathBuf::from(data_home),
        Err(e) => {
            log(&format!("db: XDG_DATA_HOME not set ({e}), falling back to HOME"));
            match std::env::var("HOME") {
                Ok(home) => PathBuf::from(home).join(".local").join("share"),
                Err(e) => {
                    log(&format!("db: HOME env var not set: {e}"));
                    return None;
                }
            }
        }
    };

    Some(base.join("termodoro").join("termodoro.sqlite3"))
}

fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}
