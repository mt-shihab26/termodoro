use std::io::Result;

use ratatui::Frame;
use ratatui::crossterm::event::KeyEvent;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Block, Paragraph};

use crate::commands::tui::tabs::Tab;

pub struct Todos;

impl Tab for Todos {
    fn name(&self) -> &str {
        "Todos [1]"
    }
    fn color(&self) -> Color {
        Color::Cyan
    }

    fn render(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Paragraph::new("Great terminal interfaces start with a single widget.")
                .alignment(Alignment::Center)
                .fg(self.color())
                .block(Block::bordered().fg(self.color())),
            area,
        );
    }

    fn handle(&mut self, _key: KeyEvent) -> Result<()> {
        Ok(())
    }
}
