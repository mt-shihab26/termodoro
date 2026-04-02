use std::io::Result;
use std::sync::OnceLock;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::migration::{Migrator, MigratorTrait};
use crate::utils::path::db_path;

static RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

pub fn rt() -> &'static tokio::runtime::Runtime {
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("failed to build tokio runtime")
    })
}

pub fn connect() -> Result<DatabaseConnection> {
    let path = db_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    crate::utils::log::init();

    rt().block_on(async {
        let url = format!("sqlite://{}?mode=rwc", path.to_str().unwrap());

        let mut opts = ConnectOptions::new(url);
        opts.sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info);

        let db = Database::connect(opts).await.map_err(io_err)?;

        Migrator::up(&db, None).await.map_err(io_err)?;

        Ok(db)
    })
}

pub fn reset() -> Result<()> {
    let path = db_path();
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    Ok(())
}

fn io_err(e: impl std::fmt::Display) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
}
