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
    /// Active search query shown inline; empty string means no search is active.
    search_query: String,
}

impl HintProps {
    /// Creates new hint props from the current mode, delete availability, and active search query.
    pub fn new(ui_mode: TodosMode, can_delete: bool, search_query: String) -> Self {
        Self { ui_mode, can_delete, search_query }
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
                let base = if self.props.can_delete {
                    "[[/]]Page  [j/k]Navigate  [Space]Toggle  [a]Add  [e]Edit  [^r]Delete  [/]Search"
                } else {
                    "[[/]]Page  [j/k]Navigate  [Space]Toggle  [a]Add  [e]Edit  [/]Search"
                };
                if !self.props.search_query.is_empty() {
                    format!("{}  \"{}\"", base, self.props.search_query)
                } else {
                    base.to_string()
                }
            }
            TodosMode::Adding | TodosMode::Editing => {
                "[Enter]Confirm  [Esc]Cancel  [Backspace]Delete char".to_string()
            }
            TodosMode::Searching => {
                "[Enter]Confirm search  [Esc]Cancel search  [Backspace]Delete char".to_string()
            }
        };

        Paragraph::new(hint).centered().fg(Color::DarkGray).render(area, buf);
    }
}
