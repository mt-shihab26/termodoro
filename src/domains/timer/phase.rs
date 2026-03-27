use ratatui::style::Color;

use super::config::Config;

#[derive(Clone, PartialEq)]
pub enum Phase {
    Work,
    Break,
    LongBreak,
}

impl Phase {
    pub fn label(&self) -> &str {
        match self {
            Phase::Work => "Work Session",
            Phase::Break => "Short Break",
            Phase::LongBreak => "Long Break",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Phase::Work => Color::Red,
            Phase::Break => Color::Green,
            Phase::LongBreak => Color::Cyan,
        }
    }

    pub fn duration(&self, config: &Config) -> u64 {
        match self {
            Phase::Work => config.work_duration(),
            Phase::Break => config.break_duration(),
            Phase::LongBreak => config.long_break_duration(),
        }
    }
}
