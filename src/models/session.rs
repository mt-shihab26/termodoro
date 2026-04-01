use std::io;

use sea_orm::{ActiveModelBehavior, DeriveEntityModel, QueryFilter, QueryOrder};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection};
use sea_orm::{DerivePrimaryKey, DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait};
use time::OffsetDateTime;

use crate::{kinds::phase::Phase, log_error, utils::db::rt};

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

#[derive(Clone)]
pub struct Session {
    pub id: Option<i32>,
    pub phase: String,
    pub duration_secs: u32,
    pub completed_at: Option<String>,
    pub todo_id: Option<i32>,
}

impl Session {
    pub fn new(phase: &Phase, duration_millis: u32, todo_id: Option<i32>, completed: bool) -> Self {
        let completed_at = if completed {
            let dt = OffsetDateTime::now_utc();
            Some(format!(
                "{}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
                dt.year(),
                dt.month() as u8,
                dt.day(),
                dt.hour(),
                dt.minute(),
                dt.second()
            ))
        } else {
            None
        };
        Self {
            id: None,
            phase: phase.to_db_str().to_string(),
            duration_secs: duration_millis / 1000,
            completed_at,
            todo_id,
        }
    }

    // TODO: Work on logic
    pub fn record(db: &DatabaseConnection, phase: &Phase, duration_millis: u32, todo_id: Option<i32>, completed: bool) {
        Self::new(phase, duration_millis, todo_id, completed).save(db);
    }

    pub fn list_for_todo(db: &DatabaseConnection, todo_id: i32) -> Vec<Session> {
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

    pub fn stats_for_todo(db: &DatabaseConnection, todo_id: i32) -> (u32, u32) {
        let sessions: Vec<_> = Self::list_for_todo(db, todo_id)
            .into_iter()
            .filter(|s| s.completed_at.is_some())
            .collect();
        let count = sessions.len() as u32;
        let total_secs: u32 = sessions.iter().map(|s| s.duration_secs).sum();
        (count, total_secs)
    }

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
