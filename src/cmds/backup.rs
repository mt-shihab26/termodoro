use std::io::Result;

use crate::{cmds::Cmd, domains::backup::run_backup};

/// Command that backs up the local database to the user's Google Drive.
pub struct Backup;

impl Backup {
    /// Creates a new `Backup` command.
    pub fn new() -> Self {
        Self
    }
}

impl Cmd for Backup {
    /// Returns the CLI name and description for the backup command.
    fn help() -> &'static [&'static str] {
        &["backup", "Back up your data to Google Drive"]
    }

    /// Uploads a gzip-compressed snapshot of the database to Google Drive.
    fn run(self: Box<Self>) -> Result<()> {
        run_backup()
    }
}
