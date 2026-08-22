use std::io::Result;

use crate::{cmds::Cmd, domains::backup::run_status};

/// Command that shows whether orivo is currently signed in to Google Drive.
pub struct Status;

impl Status {
    /// Creates a new `Status` command.
    pub fn new() -> Self {
        Self
    }
}

impl Cmd for Status {
    /// Returns the CLI name and description for the status command.
    fn help() -> &'static [&'static str] {
        &["status", "Show Google Drive sign-in status"]
    }

    /// Prints whether a Google Drive refresh token is currently stored.
    fn run(self: Box<Self>) -> Result<()> {
        run_status()
    }
}
