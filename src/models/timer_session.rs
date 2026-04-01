use std::io;

use sea_orm::{ActiveModelBehavior, ActiveModelTrait, ActiveValue::Set, DatabaseConnection};
use sea_orm::{DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EnumIter, PrimaryKeyTrait};
use time::OffsetDateTime;

use crate::kinds::phase::Phase;
use crate::utils::db::rt;
use crate::log_error;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "timer_sessions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub phase: String,
    pub duration_secs: i32,
    pub completed_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub fn record(db: &DatabaseConnection, phase: &Phase, duration_millis: u32) {
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
        ..Default::default()
    };

    match rt().block_on(async { model.insert(db).await.map_err(io_err) }) {
        Ok(_) => {}
        Err(e) => {
            log_error!("failed to save timer session: {e}");
        }
    }
}

fn io_err(e: impl std::fmt::Display) -> io::Error {
    io::Error::new(io::ErrorKind::Other, e.to_string())
}
