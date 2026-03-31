use std::io::Result;

use crate::handlers::tui::app::App;

use super::Cmd;

pub struct Tui;

impl Tui {
    pub fn new() -> Self {
        Self {}
    }
}

impl Cmd for Tui {
    fn help(&self) -> &[&str] {
        &["(default)", "tui", "Launch the terminal UI"]
    }

    fn run(&self) -> Result<()> {
        let mut app = App::new();

        app.run()
    }
}
