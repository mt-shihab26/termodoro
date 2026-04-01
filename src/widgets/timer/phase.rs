use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::style::Stylize;
use ratatui::widgets::{Paragraph, Widget};

pub struct PhaseWidget {
    pub label: String,
    pub color: Color,
}

impl Widget for &PhaseWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.label.clone())
            .centered()
            .bold()
            .fg(self.color)
            .render(area, buf);
    }
}
