use std::io::Result;

use crate::{cmd::Cmd, tui::App};

pub struct Tui;

impl Tui {
    pub fn new() -> Self {
        Self {}
    }
}

impl Cmd for Tui {
    fn help(&self) -> &str {
        "tui    Launch the terminal UI"
    }

    fn run(&self) -> Result<()> {
        let mut terminal = ratatui::init();

        let mut app = App::new(&mut terminal);

        let result = app.run();

        ratatui::restore();

        result
    }
}
