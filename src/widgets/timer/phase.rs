//! Phase label widget that displays the current timer phase (Focus, Break, etc.).

use ratatui::{
    prelude::{Buffer, Color, Rect, Stylize, Widget},
    widgets::Paragraph,
};

/// Props for the phase label widget.
pub struct PhaseProps {
    /// Display text for the current phase (e.g. "Focus", "Break").
    label: String,
    /// Accent color matching the active phase.
    color: Color,
}

impl PhaseProps {
    /// Creates new phase props with the given label and color.
    pub fn new(label: String, color: Color) -> Self {
        Self { label, color }
    }
}

/// Stateless widget that renders the current phase name.
pub struct PhaseWidget<'a> {
    /// Borrowed phase props for this render pass.
    props: &'a PhaseProps,
}

impl<'a> PhaseWidget<'a> {
    /// Creates a new phase widget from the given props.
    pub fn new(props: &'a PhaseProps) -> Self {
        Self { props }
    }
}

impl Widget for &PhaseWidget<'_> {
    /// Renders the phase label centered and bold into the buffer.
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.props.label.clone())
            .centered()
            .bold()
            .fg(self.props.color)
            .render(area, buf);
    }
}
