use std::io::Result;

use ratatui::DefaultTerminal;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{Frame, widgets::Paragraph};

pub struct App<'a> {
    alive: bool,
    terminal: &'a mut DefaultTerminal,
}

impl<'a> App<'a> {
    pub fn new(terminal: &'a mut DefaultTerminal) -> Self {
        Self { alive: true, terminal }
    }

    pub fn run(&mut self) -> Result<()> {
        while self.alive {
            self.handle_events()?;
            self.render_pixels()?;
        }

        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Char('q') => self.alive = false,
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }

    fn render_pixels(&mut self) -> Result<()> {
        self.terminal.draw(|frame| Self::render_frame(frame))?;

        Ok(())
    }

    fn render_frame(frame: &mut Frame) {
        let text = Paragraph::new("Hello World!");
        frame.render_widget(text, frame.area());
    }
}
