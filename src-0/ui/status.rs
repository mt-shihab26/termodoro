use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::Paragraph,
};

use crate::timer::Status as TimerStatus;

use super::component::Component;

pub struct StatusIndicator<'a> {
    pub status: &'a TimerStatus,
}

impl Component for StatusIndicator<'_> {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let text = match self.status {
            TimerStatus::Running => "● Running",
            TimerStatus::Paused => "⏸ Paused",
        };
        frame.render_widget(
            Paragraph::new(text)
                .style(Style::default().fg(Color::Yellow))
                .alignment(Alignment::Center),
            area,
        );
    }
}
