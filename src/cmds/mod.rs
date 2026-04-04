//! CLI command modules and the shared command trait.

pub mod help;
pub mod seed;
pub mod tui;
pub mod version;

use std::io::Result;

/// Common interface implemented by all CLI commands.
pub trait Cmd {
    /// Executes the command.
    fn run(self: Box<Self>) -> Result<()>;
    /// Returns the help entries shown in the aggregated command list.
    fn help() -> &'static [&'static str];
}
