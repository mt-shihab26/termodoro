//! Context-sensitive keyboard hint bar for the timer tab.

use ratatui::{
    prelude::{Buffer, Color, Rect, Stylize, Widget},
    widgets::Paragraph,
};

/// Props for the timer keyboard-hint bar.
pub struct HintProps {
    /// Whether the todo-picker overlay is currently open.
    selecting_todo: bool,
}

impl HintProps {
    /// Creates new hint props indicating whether todo selection is active.
    pub fn new(selecting_todo: bool) -> Self {
        Self { selecting_todo }
    }
}

/// Stateless widget that renders context-sensitive key hints for the timer.
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
        let text = if self.props.selecting_todo {
            "[j/k] Navigate   [Enter] Select   [Esc] Cancel"
        } else {
            "[Space] Toggle   [r] Reset   [n] Skip   [t] Select Todo   [T] Clear todo   [m] Millis"
        };
        Paragraph::new(text).centered().fg(Color::DarkGray).render(area, buf);
    }
}
