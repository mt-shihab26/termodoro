use std::io;

use sea_orm::{ActiveModelBehavior, DeriveEntityModel, QueryFilter, QueryOrder};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection};
use sea_orm::{DerivePrimaryKey, DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait};

use crate::{
    kinds::phase::Phase,
    log_error,
    utils::{date::now_utc_str, db::rt},
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "sessions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    id: i32,
    phase: String,
    duration_secs: i32,
    completed_at: Option<String>,
    todo_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

/// Aggregated work session statistics for a single todo.
#[derive(Clone)]
pub struct Stat {
    /// Number of completed work sessions.
    pub completed_sessions: u32,
    /// Total time spent in seconds across all completed work sessions.
    pub completed_secs: u32,
}

impl Stat {
    pub fn new(completed_sessions: u32, completed_secs: u32) -> Self {
        Self {
            completed_sessions,
            completed_secs,
        }
    }
}

/// A single pomodoro session record.
#[derive(Clone)]
pub struct Session {
    /// Database primary key, `None` before the record is saved.
    pub id: Option<i32>,
    /// Phase identifier stored as a string (e.g. `"work"`, `"break"`).
    pub phase: String,
    /// Duration of the session in seconds.
    pub duration_secs: u32,
    /// ISO 8601 UTC timestamp of when the session was completed, `None` if not yet completed.
    pub completed_at: Option<String>,
    /// Associated todo id, if any.
    pub todo_id: Option<i32>,
}

impl Session {
    /// Creates a new completed session for the given phase and duration.
    pub fn new(phase: &Phase, duration_millis: u32, todo_id: Option<i32>) -> Self {
        Self {
            id: None,
            phase: phase.to_db_str().to_string(),
            duration_secs: duration_millis / 1000,
            completed_at: Some(now_utc_str()),
            todo_id,
        }
    }

    /// Creates and persists a completed session to the database.
    pub fn record(db: &DatabaseConnection, phase: &Phase, duration_millis: u32, todo_id: Option<i32>) {
        Self::new(phase, duration_millis, todo_id).save(db);
    }

    /// Returns aggregated work session stats for the given todo.
    pub fn stat(db: &DatabaseConnection, todo_id: i32) -> Stat {
        let sessions: Vec<_> = Self::get(db, todo_id)
            .into_iter()
            .filter(|s| s.completed_at.is_some() && s.phase == Phase::Work.to_db_str())
            .collect();

        let completed_sessions = sessions.len() as u32;
        let completed_secs = sessions.iter().map(|s| s.duration_secs).sum();

        Stat::new(completed_sessions, completed_secs)
    }

    /// Fetches all sessions for the given todo from the database.
    fn get(db: &DatabaseConnection, todo_id: i32) -> Vec<Session> {
        match rt().block_on(async {
            Entity::find()
                .filter(Column::TodoId.eq(todo_id))
                .order_by_asc(Column::Id)
                .all(db)
                .await
                .map_err(io_err)
        }) {
            Ok(models) => models.into_iter().map(Session::from).collect(),
            Err(e) => {
                log_error!("failed to list sessions for todo {todo_id}: {e}");
                vec![]
            }
        }
    }

    /// Inserts or updates this session in the database.
    fn save(&mut self, db: &DatabaseConnection) -> bool {
        match rt().block_on(async { self.to_model().insert(db).await.map_err(io_err) }) {
            Ok(model) => {
                *self = model.into();
                true
            }
            Err(e) => {
                log_error!("failed to save session: {e}");
                false
            }
        }
    }

    fn to_model(&self) -> ActiveModel {
        match self.id {
            Some(id) => ActiveModel {
                id: Set(id),
                phase: Set(self.phase.clone()),
                duration_secs: Set(self.duration_secs as i32),
                completed_at: Set(self.completed_at.clone()),
                todo_id: Set(self.todo_id),
            },
            None => ActiveModel {
                phase: Set(self.phase.clone()),
                duration_secs: Set(self.duration_secs as i32),
                completed_at: Set(self.completed_at.clone()),
                todo_id: Set(self.todo_id),
                ..Default::default()
            },
        }
    }
}

impl From<Model> for Session {
    fn from(m: Model) -> Self {
        Self {
            id: Some(m.id),
            phase: m.phase,
            duration_secs: m.duration_secs.max(0) as u32,
            completed_at: m.completed_at,
            todo_id: m.todo_id,
        }
    }
}

fn io_err(e: impl std::fmt::Display) -> io::Error {
    io::Error::new(io::ErrorKind::Other, e.to_string())
}
