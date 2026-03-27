use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Block, Paragraph, Widget};

pub const COLOR: Color = Color::Cyan;

pub struct Todos;

impl Widget for Todos {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Great terminal interfaces start with a single widget.")
            .alignment(Alignment::Center)
            .fg(COLOR)
            .block(Block::bordered().fg(COLOR))
            .render(area, buf);
    }
}
