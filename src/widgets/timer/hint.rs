use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::style::Stylize;
use ratatui::widgets::{Paragraph, Widget};

pub struct HintWidget {
    pub selecting_todo: bool,
}

impl Widget for &HintWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = if self.selecting_todo {
            "[j/k] Navigate   [Enter] Select   [Esc] Cancel"
        } else {
            "[Space] Toggle   [r] Reset   [n] Skip   [t] Todo   [T] Clear todo"
        };
        Paragraph::new(text).centered().fg(Color::DarkGray).render(area, buf);
    }
}
