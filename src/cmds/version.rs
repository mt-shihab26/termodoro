use std::io::Result;

use crate::cmds::Cmd;

/// Command that prints the current application version.
pub struct Version;

impl Version {
    /// Creates a new `Version` command.
    pub fn new() -> Self {
        Self {}
    }
}

impl Cmd for Version {
    /// Returns the CLI aliases and description for the version command.
    fn help() -> &'static [&'static str] {
        &["version", "--version", "-V", "Print the current version"]
    }

    /// Prints the current package version.
    fn run(self: Box<Self>) -> Result<()> {
        println!("v{}", env!("CARGO_PKG_VERSION"));
        Ok(())
    }
}
