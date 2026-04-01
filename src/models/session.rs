use std::io;

use sea_orm::{ActiveModelBehavior, DeriveEntityModel, QueryFilter};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection};
use sea_orm::{DerivePrimaryKey, DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait};
use time::OffsetDateTime;

use crate::kinds::phase::Phase;
use crate::log_error;
use crate::utils::db::rt;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "sessions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub phase: String,
    pub duration_secs: i32,
    pub completed_at: String,
    pub todo_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub struct Session {
    pub id: Option<i32>,
    pub phase: String,
    pub duration_secs: u32,
    pub completed_at: String,
    pub todo_id: Option<i32>,
}

impl Session {
    pub fn record(
        db: &DatabaseConnection,
        phase: &Phase,
        duration_millis: u32,
        todo_id: Option<i32>,
    ) {
        let dt = OffsetDateTime::now_utc();
        let completed_at = format!(
            "{}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
            dt.year(),
            dt.month() as u8,
            dt.day(),
            dt.hour(),
            dt.minute(),
            dt.second()
        );

        let model = ActiveModel {
            phase: Set(phase.to_db_str().to_string()),
            duration_secs: Set((duration_millis / 1000) as i32),
            completed_at: Set(completed_at),
            todo_id: Set(todo_id),
            ..Default::default()
        };

        match rt().block_on(async { model.insert(db).await.map_err(io_err) }) {
            Ok(_) => {}
            Err(e) => {
                log_error!("failed to save timer session: {e}");
            }
        }
    }

    pub fn list_for_todo(db: &DatabaseConnection, todo_id: i32) -> Vec<Session> {
        match rt().block_on(async {
            Entity::find()
                .filter(Column::TodoId.eq(todo_id))
                .all(db)
                .await
                .map_err(io_err)
        }) {
            Ok(models) => models.into_iter().map(Session::from).collect(),
            Err(e) => {
                log_error!("failed to list timer sessions for todo {todo_id}: {e}");
                vec![]
            }
        }
    }

    pub fn stats_for_todo(db: &DatabaseConnection, todo_id: i32) -> (u32, u32) {
        let sessions = Self::list_for_todo(db, todo_id);
        let count = sessions.len() as u32;
        let total_secs: u32 = sessions.iter().map(|s| s.duration_secs).sum();
        (count, total_secs)
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
