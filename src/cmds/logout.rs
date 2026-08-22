use std::io::Result;

use crate::{cmds::Cmd, domains::backup::run_logout};

/// Command that signs out of Google Drive by removing the stored refresh token.
pub struct Logout;

impl Logout {
    /// Creates a new `Logout` command.
    pub fn new() -> Self {
        Self
    }
}

impl Cmd for Logout {
    /// Returns the CLI name and description for the logout command.
    fn help() -> &'static [&'static str] {
        &["logout", "Sign out of Google Drive"]
    }

    /// Removes the locally stored Google refresh token.
    fn run(self: Box<Self>) -> Result<()> {
        run_logout()
    }
}
