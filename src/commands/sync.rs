use std::io::{Error, ErrorKind, Result};

use crate::commands::Command;
use crate::db::{config::Config, store};

pub struct Sync;

impl Sync {
    pub fn new() -> Self {
        Self
    }
}

impl Command for Sync {
    fn help(&self) -> &[&str] {
        &["sync", "Sync local database to Turso cloud"]
    }

    fn run(&self) -> Result<()> {
        let config = Config::load()?;

        if config.turso.is_none() {
            eprintln!("No Turso credentials found.");
            eprintln!("Config file: {}", Config::path().display());
            eprintln!();
            eprintln!("Run `orivo backup` to generate a config template, then fill in your credentials.");
            eprintln!();
            eprintln!("Get credentials with the Turso CLI:");
            eprintln!("  turso auth login");
            eprintln!("  turso db create orivo");
            eprintln!("  turso db show orivo --url");
            eprintln!("  turso db tokens create orivo");
            return Err(Error::new(ErrorKind::NotFound, "missing Turso config"));
        }

        println!("Opening database...");
        let db = store::open()?;

        println!("Syncing to Turso...");
        store::sync(&db)?;

        println!("Done.");
        Ok(())
    }
}
