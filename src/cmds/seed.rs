use std::io::Result;

use crate::{cmds::Cmd, domains::seed::seed_todos, models::todo::Todo, utils::db};

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
        let items = seed_todos(100);
        let total = items.len();
        let mut inserted = 0usize;

        for item in items {
            if Todo::add(&db, item.text, item.due_date, item.repeat).is_some() {
                inserted += 1;
            }
        }

        println!("inserted {inserted}/{total} todos");
        Ok(())
    }
}
