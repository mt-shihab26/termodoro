use std::io::Result;

use crate::handlers::tui::App;

use super::Command;

pub struct Tui;

impl Tui {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for Tui {
    fn help(&self) -> &[&str] {
        &["(default)", "tui", "Launch the terminal UI"]
    }

    fn run(&self) -> Result<()> {
        let mut app = App::new();

        app.run()
    }
}
