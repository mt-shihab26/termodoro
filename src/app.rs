use std::io::Result;

use ratatui::Frame;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::style::Style;
use ratatui::widgets::{Block, Tabs};
use ratatui::{DefaultTerminal, symbols};

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
        let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"])
            .block(Block::bordered().title("Tabs"))
            .style(Style::default().white())
            .highlight_style(Style::default().yellow())
            .select(2)
            .divider(symbols::DOT)
            .padding("->", "<-");

        frame.render_widget(tabs, frame.area());
    }
}
