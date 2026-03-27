use std::io::Result;

use ratatui::DefaultTerminal;

pub struct App {
    alive: bool,
    terminal: &mut DefaultTerminal,
}

impl App {
    pub fn new(terminal: &mut DefaultTerminal) -> Self {
        Self { alive: true, terminal }
    }

    pub fn run() -> Result<()> {
        Ok(())
    }
}
