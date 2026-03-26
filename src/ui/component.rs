use ratatui::{Frame, layout::Rect};
use ratatui::crossterm::event::KeyEvent;

use crate::state::Phase;
use ratatui::style::Color;

pub trait Component {
    fn render(&self, frame: &mut Frame, area: Rect);
    fn handle_key(&mut self, _key: KeyEvent) {}
    fn on_tick(&mut self) {}
}

pub fn phase_color(phase: &Phase) -> Color {
    match phase {
        Phase::Work => Color::Red,
        Phase::Break => Color::Green,
        Phase::LongBreak => Color::Blue,
    }
}
