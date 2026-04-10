use ratatui::{
    prelude::{Buffer, Color, Rect, Stylize, Widget},
    widgets::Paragraph,
};

/// Props for the timer running/paused status widget.
pub struct StatusProps {
    /// Whether the timer is currently running.
    running: bool,
    /// Pass the phase color to use in everywhere
    color: Color,
}

impl StatusProps {
    /// Creates new status props from the current running state.
    pub fn new(running: bool, color: Color) -> Self {
        Self { running, color }
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
            ("Running", self.props.color)
        } else {
            ("Paused", Color::DarkGray)
        };
        Paragraph::new(label).centered().fg(color).render(area, buf);
    }
}
