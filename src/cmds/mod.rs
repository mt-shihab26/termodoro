/// Command that backs up the local database to Google Drive.
pub mod backup;
/// Help command that prints usage for all available commands.
pub mod help;
/// Command that signs in to Google Drive without backing up or restoring.
pub mod login;
/// Command that restores the local database from the Google Drive backup.
pub mod restore;
/// Development command that resets and seeds the database (excluded from release builds).
#[cfg(debug_assertions)]
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
