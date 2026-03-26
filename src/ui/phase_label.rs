use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    widgets::Paragraph,
};

use crate::state::Phase;

use super::component::{Component, phase_color};

pub struct PhaseLabel<'a> {
    pub phase: &'a Phase,
}

impl Component for PhaseLabel<'_> {
    fn render(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Paragraph::new(self.phase.label())
                .style(
                    Style::default()
                        .fg(phase_color(self.phase))
                        .add_modifier(Modifier::BOLD),
                )
                .alignment(Alignment::Center),
            area,
        );
    }
}
