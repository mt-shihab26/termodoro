use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::style::Stylize;
use ratatui::widgets::{Paragraph, Widget};

pub struct PhaseProps {
    label: String,
    color: Color,
}

impl PhaseProps {
    pub fn new(label: String, color: Color) -> Self {
        Self { label, color }
    }
}

pub struct PhaseWidget<'a> {
    props: &'a PhaseProps,
}

impl<'a> PhaseWidget<'a> {
    pub fn new(props: &'a PhaseProps) -> Self {
        Self { props }
    }
}

impl Widget for &PhaseWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.props.label.clone())
            .centered()
            .bold()
            .fg(self.props.color)
            .render(area, buf);
    }
}
