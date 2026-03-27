mod tabs;
mod workers;

pub mod app;
pub mod event;
pub mod fps;

use std::io::Result;

use crate::{Cmd, tui::app::App};

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
