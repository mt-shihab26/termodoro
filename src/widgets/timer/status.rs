//! Running/paused status label widget for the timer tab.

use ratatui::{
    prelude::{Buffer, Color, Rect, Stylize, Widget},
    widgets::Paragraph,
};

use crate::kinds::phase::COLOR;

/// Props for the timer running/paused status widget.
pub struct StatusProps {
    /// Whether the timer is currently running.
    running: bool,
}

impl StatusProps {
    /// Creates new status props from the current running state.
    pub fn new(running: bool) -> Self {
        Self { running }
    }
}

/// Stateless widget that renders "Running" or "Paused".
pub struct StatusWidget<'a> {
    /// Borrowed status props for this render pass.
    props: &'a StatusProps,
}

impl<'a> StatusWidget<'a> {
    /// Creates a new status widget from the given props.
    pub fn new(props: &'a StatusProps) -> Self {
        Self { props }
    }
}

impl Widget for &StatusWidget<'_> {
    /// Renders the running/paused label centered into the buffer.
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (label, color) = if self.props.running {
            ("Running", COLOR)
        } else {
            ("Paused", Color::DarkGray)
        };
        Paragraph::new(label).centered().fg(color).render(area, buf);
    }
}
