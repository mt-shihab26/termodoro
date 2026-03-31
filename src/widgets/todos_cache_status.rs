use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Paragraph, Widget};

pub struct TodosCacheStatusWidget {
    pub status: String,
}

impl Widget for TodosCacheStatusWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.status).fg(Color::DarkGray).right_aligned().render(
            Rect {
                x: area.x + 1,
                y: area.y,
                width: area.width.saturating_sub(2),
                height: 1,
            },
            buf,
        );
    }
}
