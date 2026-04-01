use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::style::Stylize;
use ratatui::widgets::{Paragraph, Widget};

pub struct SessionWidget {
    pub session: u32,
    pub total: u32,
}

impl Widget for &SessionWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(format!("Session {} / {}", self.session, self.total))
            .centered()
            .fg(Color::DarkGray)
            .render(area, buf);
    }
}
