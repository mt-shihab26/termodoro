use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Paragraph, Widget};

use crate::kinds::{mode::Mode, page::Page};

pub struct HintWidget {
    pub page: Page,
    pub ui_mode: Mode,
    pub can_delete: bool,
}

impl Widget for &HintWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let hint = match self.ui_mode {
            Mode::Normal => match self.page {
                Page::History => "[[/]]Page  [j/k]Navigate",
                _ if self.can_delete => {
                    "[[/]]Page  [j/k]Navigate  [Space]Toggle  [a]Add  [e]Edit  [^d]Delete"
                }
                _ => "[[/]]Page  [j/k]Navigate  [Space]Toggle  [a]Add  [e]Edit",
            },
            Mode::Adding | Mode::Editing => "[Enter]Confirm  [Esc]Cancel  [Backspace]Delete char",
        };

        Paragraph::new(hint)
            .centered()
            .fg(Color::DarkGray)
            .render(area, buf);
    }
}
