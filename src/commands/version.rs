use std::io::Result;

use crate::commands::Command;

pub struct Version;

impl Version {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for Version {
    fn help(&self) -> &[&str] {
        &["version", "--version", "-V", "Print the current version"]
    }

    fn run(&self) -> Result<()> {
        println!("{}", env!("CARGO_PKG_VERSION"));
        Ok(())
    }
}
