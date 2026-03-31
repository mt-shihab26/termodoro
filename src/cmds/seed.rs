use std::io::Result;

use crate::cmds::Cmd;
use crate::domains::seed::seed_todos;
use crate::utils::db;

pub struct Seed;

impl Seed {
    pub fn new() -> Self {
        Self
    }
}

impl Cmd for Seed {
    fn help() -> &'static [&'static str] {
        &["seed", "Seed the database with fake todos for development"]
    }

    fn run(self: Box<Self>) -> Result<()> {
        db::reset()?;
        let db = db::connect()?;
        let items = seed_todos();
        let total = items.len();
        let mut inserted = 0usize;

        for mut todo in items {
            if todo.save(&db) {
                inserted += 1;
            }
        }

        println!("inserted {inserted}/{total} todos");
        Ok(())
    }
}
