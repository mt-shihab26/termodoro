use std::io;

use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DeriveEntityModel,
    DerivePrimaryKey, DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait, QueryFilter, QueryOrder,
};
use time::Date;

use crate::{
    kinds::phase::Phase,
    log_error,
    utils::{
        date::{format_date, now_utc_str, parse_date, today},
        db::rt,
    },
};

/// Aggregated work session statistics for a single todo.
#[derive(Clone)]
pub struct Stat {
    /// Number of completed work sessions.
    pub completed_sessions: u32,
    /// Total time spent in seconds across all completed work sessions.
    pub completed_secs: u32,
}

impl Stat {
    /// Creates a new `Stat` with the given session count and total seconds.
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
    /// UTC timestamp of when the session started, `None` if not yet started.
    pub started_at: Option<String>,
    /// UTC timestamp of when the session ended, `None` if not yet completed.
    pub ended_at: Option<String>,
    /// Associated todo id, if any.
    pub todo_id: Option<i32>,
    /// Date when the record was created.
    pub created_at: Date,
    /// Date when the record was last updated.
    pub updated_at: Date,
}

impl Session {
    /// Creates a new completed session for the given phase and duration.
    pub fn new(phase: &Phase, duration_millis: u32, started_at: Option<String>, todo_id: Option<i32>) -> Self {
        Self {
            id: None,
            phase: phase.to_db_str().to_string(),
            duration_secs: duration_millis / 1000,
            started_at,
            ended_at: Some(now_utc_str()),
            todo_id,
            created_at: today(),
            updated_at: today(),
        }
    }

    /// Creates and persists a completed session to the database.
    pub fn record(
        db: &DatabaseConnection,
        phase: &Phase,
        duration_millis: u32,
        started_at: Option<String>,
        todo_id: Option<i32>,
    ) {
        Self::new(phase, duration_millis, started_at, todo_id).save(db);
    }

    /// Returns the number of completed work sessions started today.
    pub fn count_today(db: &DatabaseConnection) -> u32 {
        let prefix = format!("{}%", format_date(today()));
        match rt().block_on(async {
            Entity::find()
                .filter(Column::Phase.eq(Phase::Work.to_db_str()))
                .filter(Column::EndedAt.like(prefix))
                .all(db)
                .await
                .map_err(io_err)
        }) {
            Ok(rows) => rows.len() as u32,
            Err(e) => {
                log_error!("failed to count today's sessions: {e}");
                0
            }
        }
    }

    /// Returns aggregated work session stats for the given todo.
    pub fn stat(db: &DatabaseConnection, todo_id: i32) -> Stat {
        let sessions: Vec<_> = Self::get(db, todo_id)
            .into_iter()
            .filter(|s| s.ended_at.is_some() && s.phase == Phase::Work.to_db_str())
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

    /// Converts this session into a SeaORM active model for insert or update.
    fn to_model(&self) -> ActiveModel {
        let today = format_date(today());
        match self.id {
            Some(id) => ActiveModel {
                id: Set(id),
                phase: Set(self.phase.clone()),
                duration_secs: Set(self.duration_secs as i32),
                started_at: Set(self.started_at.clone()),
                ended_at: Set(self.ended_at.clone()),
                todo_id: Set(self.todo_id),
                created_at: Set(format_date(self.created_at)),
                updated_at: Set(today),
            },
            None => ActiveModel {
                phase: Set(self.phase.clone()),
                duration_secs: Set(self.duration_secs as i32),
                started_at: Set(self.started_at.clone()),
                ended_at: Set(self.ended_at.clone()),
                todo_id: Set(self.todo_id),
                created_at: Set(today.clone()),
                updated_at: Set(today),
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
            started_at: m.started_at,
            ended_at: m.ended_at,
            todo_id: m.todo_id,
            created_at: parse_date(&m.created_at).unwrap_or_else(today),
            updated_at: parse_date(&m.updated_at).unwrap_or_else(today),
        }
    }
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "sessions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    id: i32,
    phase: String,
    duration_secs: i32,
    started_at: Option<String>,
    ended_at: Option<String>,
    todo_id: Option<i32>,
    created_at: String,
    updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

fn io_err(e: impl std::fmt::Display) -> io::Error {
    io::Error::new(io::ErrorKind::Other, e.to_string())
}
