use std::io::Result;

use crate::cmd::Cmd;

pub struct Version;

impl Version {
    pub fn new() -> Self {
        Self {}
    }
}

impl Cmd for Version {
    fn help(&self) -> &str {
        "version    Print the current version"
    }

    fn run(&self) -> Result<()> {
        println!("{}", env!("CARGO_PKG_VERSION"));
        Ok(())
    }
}
