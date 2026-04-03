use ratatui::{
    prelude::{Buffer, Color, Rect, Stylize, Widget},
    widgets::Paragraph,
};

use crate::kinds::phase::COLOR;

pub struct StatusProps {
    running: bool,
}

impl StatusProps {
    pub fn new(running: bool) -> Self {
        Self { running }
    }
}

pub struct StatusWidget<'a> {
    props: &'a StatusProps,
}

impl<'a> StatusWidget<'a> {
    pub fn new(props: &'a StatusProps) -> Self {
        Self { props }
    }
}

impl Widget for &StatusWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (label, color) = if self.props.running {
            ("Running", COLOR)
        } else {
            ("Paused", Color::DarkGray)
        };
        Paragraph::new(label).centered().fg(color).render(area, buf);
    }
}
