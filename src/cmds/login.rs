use std::io::Result;

use crate::{cmds::Cmd, domains::backup::run_login};

/// Command that runs the Google device-code sign-in flow and stores the resulting
/// refresh token, so later `backup`/`restore` runs don't need to prompt for it.
pub struct Login;

impl Login {
    /// Creates a new `Login` command.
    pub fn new() -> Self {
        Self
    }
}

impl Cmd for Login {
    /// Returns the CLI name and description for the login command.
    fn help() -> &'static [&'static str] {
        &[
            "login",
            "Sign in to Google Drive and save credentials for later backup/restore",
        ]
    }

    /// Runs the device-code sign-in flow, storing the resulting refresh token.
    fn run(self: Box<Self>) -> Result<()> {
        run_login()
    }
}
