pub use sea_orm_migration::MigratorTrait;
use sea_orm_migration::prelude::*;

mod m20250101_000001_create_todos;
mod m20260101_000002_create_timer_sessions;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250101_000001_create_todos::Migration),
            Box::new(m20260101_000002_create_timer_sessions::Migration),
        ]
    }
}
