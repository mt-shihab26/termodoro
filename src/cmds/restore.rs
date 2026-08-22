use std::io::Result;

use crate::{cmds::Cmd, domains::backup::run_restore};

/// Command that restores the local database from the Google Drive backup.
pub struct Restore;

impl Restore {
    /// Creates a new `Restore` command.
    pub fn new() -> Self {
        Self
    }
}

impl Cmd for Restore {
    /// Returns the CLI name and description for the restore command.
    fn help() -> &'static [&'static str] {
        &[
            "restore",
            "Restore your data from Google Drive (destructive: overwrites local data)",
        ]
    }

    /// Downloads the Google Drive backup and overwrites the local database with it, after
    /// an interactive confirmation prompt.
    fn run(self: Box<Self>) -> Result<()> {
        run_restore()
    }
}
