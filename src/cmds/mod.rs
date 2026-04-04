/// Help command that prints usage for all available commands.
pub mod help;
/// Development command that resets and seeds the database.
pub mod seed;
/// Command that launches the terminal UI.
pub mod tui;
/// Command that prints the application version.
pub mod version;

use std::io::Result;

/// Common interface implemented by all CLI commands.
pub trait Cmd {
    /// Executes the command.
    fn run(self: Box<Self>) -> Result<()>;
    /// Returns the help entries shown in the aggregated command list.
    fn help() -> &'static [&'static str];
}
