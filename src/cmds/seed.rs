use std::io::Result;

use crate::{cmds::Cmd, domains::seed::seed_todos, utils::db};

/// Command that resets the database and inserts sample todos.
pub struct Seed;

impl Seed {
    /// Creates a new `Seed` command.
    pub fn new() -> Self {
        Self
    }
}

impl Cmd for Seed {
    /// Returns the CLI name and description for the seed command.
    fn help() -> &'static [&'static str] {
        &["seed", "Seed the database with fake todos for development"]
    }

    /// Resets the database, seeds todos, and prints the number inserted.
    fn run(self: Box<Self>) -> Result<()> {
        db::reset()?;
        let db = db::connect()?;
        let inserted = seed_todos(200, &db);
        println!("inserted {inserted} todos");
        Ok(())
    }
}
