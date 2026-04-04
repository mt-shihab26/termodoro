//! Database migrations for creating the todos and sessions tables.

/// Re-export of the SeaORM migrator trait used by the app entrypoint.
pub use sea_orm_migration::MigratorTrait;
use sea_orm_migration::prelude::*;

mod m_00_create_todos;
mod m_01_create_sessions;

/// Registers the ordered set of database migrations for the application.
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    /// Returns all migrations in the order they must be applied.
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m_00_create_todos::Migration),
            Box::new(m_01_create_sessions::Migration),
        ]
    }
}
