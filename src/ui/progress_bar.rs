use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Style},
    widgets::Gauge,
};

use crate::state::Phase;

use super::component::{Component, phase_color};

pub struct ProgressBar<'a> {
    pub elapsed: u64,
    pub total: u64,
    pub phase: &'a Phase,
}

impl Component for ProgressBar<'_> {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let ratio = if self.total > 0 {
            (self.elapsed as f64 / self.total as f64).clamp(0.0, 1.0)
        } else {
            0.0
        };

        let [_, center, _] = ratatui::layout::Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Max(40),
            Constraint::Fill(1),
        ])
        .areas(area);

        frame.render_widget(
            Gauge::default()
                .gauge_style(
                    Style::default()
                        .fg(phase_color(self.phase))
                        .bg(Color::DarkGray),
                )
                .ratio(ratio)
                .label(""),
            center,
        );
    }
}
