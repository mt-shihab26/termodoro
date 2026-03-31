use std::io::Result;

use sea_orm::{Database, DatabaseConnection};

use crate::db::store::{db_path, rt};
use crate::migration::{Migrator, MigratorTrait};

fn io_err(e: impl std::fmt::Display) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
}

pub fn connect() -> Result<DatabaseConnection> {
    let path = db_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    rt().block_on(async {
        let url = format!("sqlite://{}?mode=rwc", path.to_str().unwrap());
        let db = Database::connect(&url).await.map_err(io_err)?;
        Migrator::up(&db, None).await.map_err(io_err)?;
        Ok(db)
    })
}
