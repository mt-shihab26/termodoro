use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::style::Stylize;
use ratatui::widgets::{Paragraph, Widget};

pub struct SessionProps {
    sessions: u32,
    long_break_interval: u32,
}

impl SessionProps {
    pub fn new(sessions: u32, long_break_interval: u32) -> Self {
        Self {
            sessions,
            long_break_interval,
        }
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
            self.props.sessions + 1,
            self.props.long_break_interval
        ))
        .centered()
        .fg(Color::DarkGray)
        .render(area, buf);
    }
}
