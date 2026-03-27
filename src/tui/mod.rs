mod tabs;
mod ui;

use std::io::Result;

use ratatui::DefaultTerminal;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};

use self::ui::Ui;

pub struct App<'a> {
    alive: bool,
    terminal: &'a mut DefaultTerminal,
    selected_tab: usize,
}

impl<'a> App<'a> {
    pub fn new(terminal: &'a mut DefaultTerminal) -> Self {
        Self {
            alive: true,
            terminal,
            selected_tab: 0,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        while self.alive {
            self.render_pixels()?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Char('q') => self.alive = false,
                KeyCode::Char('1') => self.selected_tab = 0,
                KeyCode::Char('2') => self.selected_tab = 1,
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }

    fn render_pixels(&mut self) -> Result<()> {
        let selected_tab = self.selected_tab;
        self.terminal.draw(|frame| {
            frame.render_widget(Ui { selected_tab }, frame.area());
        })?;
        Ok(())
    }
}
