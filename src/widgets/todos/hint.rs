use ratatui::{
    prelude::{Buffer, Color, Rect, Stylize, Widget},
    widgets::Paragraph,
};

use crate::kinds::todos_mode::TodosMode;

pub struct HintProps {
    ui_mode: TodosMode,
    can_delete: bool,
}

impl HintProps {
    pub fn new(ui_mode: TodosMode, can_delete: bool) -> Self {
        Self { ui_mode, can_delete }
    }
}

pub struct HintWidget<'a> {
    props: &'a HintProps,
}

impl<'a> HintWidget<'a> {
    pub fn new(props: &'a HintProps) -> Self {
        Self { props }
    }
}

impl Widget for &HintWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let hint = match self.props.ui_mode {
            TodosMode::Normal => {
                if self.props.can_delete {
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
