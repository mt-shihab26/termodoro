use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::Paragraph,
};

use crate::timer::Status;

use super::component::Component;

pub struct TimerDisplay<'a> {
    pub remaining_secs: u64,
    pub status: &'a Status,
}

impl Component for TimerDisplay<'_> {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let mins = self.remaining_secs / 60;
        let secs = self.remaining_secs % 60;
        let color = if *self.status == Status::Paused {
            Color::DarkGray
        } else {
            Color::White
        };
        frame.render_widget(
            Paragraph::new(format!("{:02}:{:02}", mins, secs))
                .style(Style::default().fg(color).add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center),
            area,
        );
    }
}
