use std::io::Result;

use crate::{cmd::Cmd, tui::app::App};

pub struct Tui;

impl Cmd for Tui {
    fn run() -> Result<()> {
        let mut terminal = ratatui::init();

        let mut app = App::new(&mut terminal);

        let result = app.run();

        ratatui::restore();

        result
    }
}
