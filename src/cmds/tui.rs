use std::io::Result;

use sea_orm::DatabaseConnection;

use crate::{config::Config, domains::tui::App};

use super::Cmd;

/// Command that launches the terminal UI.
pub struct Tui {
    /// Application configuration.
    config: Config,
    /// Database connection passed through to the app.
    db: DatabaseConnection,
}

impl Tui {
    /// Creates a new `Tui` command with the given config and database connection.
    pub fn new(config: Config, db: DatabaseConnection) -> Self {
        Self { config, db }
    }
}

impl Cmd for Tui {
    /// Returns the CLI aliases and description for the TUI command.
    fn help() -> &'static [&'static str] {
        &["(default)", "tui", "Launch the terminal UI"]
    }

    /// Starts the terminal application event loop.
    fn run(self: Box<Self>) -> Result<()> {
        let mut app = App::new(self.config, self.db);

        app.run()
    }
}
