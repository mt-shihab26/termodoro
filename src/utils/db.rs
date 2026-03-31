use std::io::Result;

use sea_orm::{Database, DatabaseConnection};

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

use std::env;
use std::path::PathBuf;
use std::sync::OnceLock;

static RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

pub fn rt() -> &'static tokio::runtime::Runtime {
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("failed to build tokio runtime")
    })
}

pub fn db_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home)
        .join(".local")
        .join("share")
        .join("orivo")
        .join("orivo.db")
}
