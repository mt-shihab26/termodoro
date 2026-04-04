use ratatui::{
    prelude::{Buffer, Color, Rect, Stylize, Widget},
    widgets::Paragraph,
};

use crate::kinds::todos_mode::TodosMode;

/// Props for the todos keyboard-hint bar.
pub struct HintProps {
    /// Current UI mode determining which hints to display.
    ui_mode: TodosMode,
    /// Whether the delete action is available for the selected todo.
    can_delete: bool,
}

impl HintProps {
    /// Creates new hint props from the current mode and delete availability.
    pub fn new(ui_mode: TodosMode, can_delete: bool) -> Self {
        Self { ui_mode, can_delete }
    }
}

/// Stateless widget that renders context-sensitive key hints for the todos tab.
pub struct HintWidget<'a> {
    /// Borrowed hint props for this render pass.
    props: &'a HintProps,
}

impl<'a> HintWidget<'a> {
    /// Creates a new hint widget from the given props.
    pub fn new(props: &'a HintProps) -> Self {
        Self { props }
    }
}

impl Widget for &HintWidget<'_> {
    /// Renders the appropriate hint text centered in the buffer.
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
