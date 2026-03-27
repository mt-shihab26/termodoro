use std::io::Result;

use ratatui::Frame;
use ratatui::crossterm::event::KeyEvent;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Block, Paragraph};

use crate::tui::tabs::Tab;

pub const COLOR: Color = Color::Cyan;

pub struct Todos;

impl Tab for Todos {
    fn name(&self) -> &str {
        "Todos"
    }
    fn color(&self) -> Color {
        COLOR
    }

    fn render(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Paragraph::new("Great terminal interfaces start with a single widget.")
                .alignment(Alignment::Center)
                .fg(COLOR)
                .block(Block::bordered().fg(COLOR)),
            area,
        );
    }

    fn handle(&mut self, _key: KeyEvent) -> Result<()> {
        Ok(())
    }
}
