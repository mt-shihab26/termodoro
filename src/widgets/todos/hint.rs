use ratatui::{
    prelude::{Buffer, Color, Rect, Stylize, Widget},
    widgets::Paragraph,
};

use crate::kinds::{page::Page, todos_mode::TodosMode};

pub struct HintWidget {
    pub page: Page,
    pub ui_mode: TodosMode,
    pub can_delete: bool,
}

impl Widget for &HintWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let hint = match self.ui_mode {
            TodosMode::Normal => {
                if self.can_delete {
                    "[[/]]Page  [j/k]Navigate  [Space]Toggle  [a]Add  [e]Edit  [^d]Delete"
                } else {
                    "[[/]]Page  [j/k]Navigate  [Space]Toggle  [a]Add  [e]Edit"
                }
            }
            TodosMode::Adding | TodosMode::Editing => "[Enter]Confirm  [Esc]Cancel  [Backspace]Delete char",
        };

        Paragraph::new(hint).centered().fg(Color::DarkGray).render(area, buf);
    }
}
