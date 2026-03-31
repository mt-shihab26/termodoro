use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Paragraph, Widget};

use crate::kinds::{page::Page, ui_mode::UiMode};

pub struct TodosHintWidget {
    pub page: Page,
    pub ui_mode: UiMode,
}

impl Widget for TodosHintWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let hint = match self.ui_mode {
            UiMode::Normal => match self.page {
                Page::History => "[[/]]Page  [j/k]Navigate",
                _ => "[[/]]Page  [j/k]Navigate  [Space]Toggle  [a]Add  [e]Edit  [^d]Delete",
            },
            UiMode::Adding | UiMode::Editing => "[Enter]Confirm  [Esc]Cancel  [Backspace]Delete char",
        };

        Paragraph::new(hint).centered().fg(Color::DarkGray).render(area, buf);
    }
}
