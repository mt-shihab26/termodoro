use std::io::Result;

use crate::cmds::Cmd;

pub struct Version;

impl Version {
    pub fn new() -> Self {
        Self {}
    }

    pub fn help() -> &'static [&'static str] {
        &["version", "--version", "-V", "Print the current version"]
    }
}

impl Cmd for Version {
    fn help(&self) -> &[&str] {
        Self::help()
    }

    fn run(self: Box<Self>) -> Result<()> {
        println!("{}", env!("CARGO_PKG_VERSION"));
        Ok(())
    }
}
