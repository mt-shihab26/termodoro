use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::style::Stylize;
use ratatui::widgets::{Paragraph, Widget};

pub struct SessionProps {
    session: u32,
    total: u32,
}

impl SessionProps {
    pub fn new(session: u32, total: u32) -> Self {
        Self { session, total }
    }
}

pub struct SessionWidget<'a> {
    props: &'a SessionProps,
}

impl<'a> SessionWidget<'a> {
    pub fn new(props: &'a SessionProps) -> Self {
        Self { props }
    }
}

impl Widget for &SessionWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(format!(
            "Session {} / {}",
            self.props.session + 1,
            self.props.total
        ))
        .centered()
        .fg(Color::DarkGray)
        .render(area, buf);
    }
}
