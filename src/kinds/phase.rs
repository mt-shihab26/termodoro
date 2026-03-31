use ratatui::style::Color;

use crate::config::timer::TimerConfig;

pub const COLOR: Color = Color::Red;

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
            Phase::Work => COLOR,
            Phase::Break => Color::Green,
            Phase::LongBreak => Color::Cyan,
        }
    }

    pub fn duration(&self, timer_config: &TimerConfig) -> u32 {
        match self {
            Phase::Work => timer_config.work_duration(),
            Phase::Break => timer_config.break_duration(),
            Phase::LongBreak => timer_config.long_break_duration(),
        }
    }
}
