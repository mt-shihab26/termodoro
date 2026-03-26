use ratatui::{Frame, layout::Rect};

use crate::state::Phase;
use ratatui::style::Color;

pub trait Component {
    fn render(&self, frame: &mut Frame, area: Rect);
}

pub fn phase_color(phase: &Phase) -> Color {
    match phase {
        Phase::Work => Color::Red,
        Phase::Break => Color::Green,
        Phase::LongBreak => Color::Blue,
    }
}
