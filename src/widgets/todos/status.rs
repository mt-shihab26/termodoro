use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Paragraph, Widget};

pub struct StatusWidget {
    pub total: usize,
    pub from: usize,
    pub to: usize,
    pub page: usize,
    pub selected_id: Option<i32>,
}

impl StatusWidget {
    pub fn new(total: usize, from: usize, to: usize, page: usize, selected_id: Option<i32>) -> Self {
        Self {
            total,
            from,
            to,
            page,
            selected_id,
        }
    }
}

impl Widget for StatusWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let status = match self.selected_id {
            Some(id) => format!(
                " items {}-{} / {}  page {}  id {} ",
                self.from, self.to, self.total, self.page, id
            ),
            None if self.total > 0 => {
                format!(" items {}-{} / {}  page {} ", self.from, self.to, self.total, self.page)
            }
            None => " items 0-0 / 0  page 1 ".to_string(),
        };

        Paragraph::new(status)
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
