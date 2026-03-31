use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Paragraph, Widget};

pub struct StatusWidget {
    pub total: usize,
    pub from: usize,
    pub to: usize,
    pub page: usize,
}

impl StatusWidget {
    pub fn new(total: usize, from: usize, to: usize, page: usize) -> Self {
        Self {
            total,
            from,
            to,
            page,
        }
    }
}

impl Widget for StatusWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(format!(
            "{} | {}-{} | {}/{}",
            self.page,
            self.from,
            self.to,
            self.to - self.from,
            self.total,
        ))
        .fg(Color::DarkGray)
        .right_aligned()
        .render(
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
