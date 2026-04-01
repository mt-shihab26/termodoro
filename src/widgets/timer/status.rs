use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::style::Stylize;
use ratatui::widgets::{Paragraph, Widget};

pub struct StatusWidget {
    pub running: bool,
}

impl Widget for &StatusWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (label, color) = if self.running {
            ("Running", Color::Green)
        } else {
            ("Paused", Color::DarkGray)
        };
        Paragraph::new(label)
            .centered()
            .fg(color)
            .render(area, buf);
    }
}
