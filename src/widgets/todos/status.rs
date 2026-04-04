use ratatui::{
    prelude::{Buffer, Color, Rect, Stylize, Widget},
    widgets::Paragraph,
};

pub struct StatusWidget {
    pub total: usize,
    pub from: usize,
    pub to: usize,
    pub page: usize,
}

impl StatusWidget {
    pub fn new(total: usize, from: usize, to: usize, page: usize) -> Self {
        Self { total, from, to, page }
    }
}

impl Widget for &StatusWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(format!(
            "Page {} • Range {}-{} • Showing {}/{} items",
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
