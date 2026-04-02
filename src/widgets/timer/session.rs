use ratatui::{
    prelude::{Buffer, Color, Rect, Stylize, Widget},
    widgets::Paragraph,
};

pub struct SessionProps {
    sessions: u32,
    daily_session_goal: u32,
}

impl SessionProps {
    pub fn new(sessions: u32, daily_session_goal: u32) -> Self {
        Self {
            sessions,
            daily_session_goal,
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
            self.props.daily_session_goal
        ))
        .centered()
        .fg(Color::DarkGray)
        .render(area, buf);
    }
}
