use std::env;
use std::fs;
use std::io::{Error, ErrorKind, Result};
use std::path::PathBuf;
use std::sync::OnceLock;

use libsql::{Builder, Database};

use crate::config::db::DBConfig;

static RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

pub fn rt() -> &'static tokio::runtime::Runtime {
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("failed to build tokio runtime")
    })
}

fn io_err(e: impl std::fmt::Display) -> Error {
    Error::new(ErrorKind::Other, e.to_string())
}

pub fn db_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home)
        .join(".local")
        .join("share")
        .join("orivo")
        .join("orivo.db")
}

/// Opens the database.
/// - If Turso credentials are configured: opens an embedded replica (local file + Turso cloud).
/// - Otherwise: opens a plain local SQLite file.
pub fn open(db_config: &DBConfig) -> Result<Database> {
    let path = db_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    rt().block_on(async {
        let db = if db_config.is_configured() {
            Builder::new_remote_replica(path.to_str().unwrap(), db_config.url.clone(), db_config.token.clone())
                .build()
                .await
                .map_err(io_err)?
        } else {
            Builder::new_local(path.to_str().unwrap())
                .build()
                .await
                .map_err(io_err)?
        };

        Ok(db)
    })
}

/// Syncs the embedded replica with Turso cloud.
/// Pushes local changes up and pulls any remote changes down.
pub fn sync(db: &Database) -> Result<()> {
    rt().block_on(async { db.sync().await.map_err(io_err).map(|_| ()) })
}
