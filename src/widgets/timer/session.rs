use ratatui::{
    prelude::{Buffer, Rect, Stylize, Widget},
    widgets::Paragraph,
};

use crate::kinds::phase::COLOR;

/// Props for the session progress widget.
pub struct SessionProps {
    /// Number of completed sessions so far today.
    sessions: u32,
    /// Target number of sessions for the day.
    daily_session_goal: u32,
}

impl SessionProps {
    /// Creates new session props with the current count and daily goal.
    pub fn new(sessions: u32, daily_session_goal: u32) -> Self {
        Self {
            sessions,
            daily_session_goal,
        }
    }
}

/// Stateless widget that renders "Session X / Y" progress text.
pub struct SessionWidget<'a> {
    /// Borrowed session props for this render pass.
    props: &'a SessionProps,
}

impl<'a> SessionWidget<'a> {
    /// Creates a new session widget from the given props.
    pub fn new(props: &'a SessionProps) -> Self {
        Self { props }
    }
}

impl Widget for &SessionWidget<'_> {
    /// Renders the session count centered into the buffer.
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(format!(
            "Session {} / {}",
            self.props.sessions, self.props.daily_session_goal
        ))
        .centered()
        .fg(COLOR)
        .render(area, buf);
    }
}
