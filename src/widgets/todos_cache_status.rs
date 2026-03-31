use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Paragraph, Widget};

pub struct TodosCacheStatusWidget {
    pub length: usize,
    pub selected_id: Option<i32>,
}

impl TodosCacheStatusWidget {
    pub fn new(length: usize, selected_id: Option<i32>) -> Self {
        Self { length, selected_id }
    }
}

impl Widget for TodosCacheStatusWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let status = match self.selected_id {
            Some(id) => format!(" loaded {}  id {} ", self.length, id),
            None => format!(" loaded {} ", self.length),
        };

        Paragraph::new(status).fg(Color::DarkGray).right_aligned().render(
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
