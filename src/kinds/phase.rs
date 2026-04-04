//! Pomodoro phase types, display metadata, and duration helpers.

use ratatui::style::Color;
use serde::{Deserialize, Serialize};

use crate::config::timer::TimerConfig;

/// Accent color used for the work phase across the UI.
pub const COLOR: Color = Color::Red;

/// A phase in the pomodoro cycle.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Phase {
    /// A focused work session.
    Work,
    /// A short break between work sessions.
    Break,
    /// A longer break after completing a full interval of work sessions.
    LongBreak,
}

impl Phase {
    /// Returns the human-readable label for the phase.
    pub fn label(&self) -> &str {
        match self {
            Phase::Work => "Work Session",
            Phase::Break => "Short Break",
            Phase::LongBreak => "Long Break",
        }
    }

    /// Returns the accent color for the phase.
    pub fn color(&self) -> Color {
        match self {
            Phase::Work => COLOR,
            Phase::Break => Color::Green,
            Phase::LongBreak => Color::Cyan,
        }
    }

    /// Returns the database string identifier for the phase.
    pub fn to_db_str(&self) -> &str {
        match self {
            Phase::Work => "work",
            Phase::Break => "break",
            Phase::LongBreak => "long_break",
        }
    }

    /// Parses a database string into a `Phase`, returning `None` if unrecognized.
    pub fn from_db_str(s: &str) -> Option<Phase> {
        match s {
            "work" => Some(Phase::Work),
            "break" => Some(Phase::Break),
            "long_break" => Some(Phase::LongBreak),
            _ => None,
        }
    }

    /// Returns the configured duration for this phase in milliseconds.
    pub fn duration(&self, timer_config: &TimerConfig) -> u32 {
        match self {
            Phase::Work => timer_config.work_duration(),
            Phase::Break => timer_config.break_duration(),
            Phase::LongBreak => timer_config.long_break_duration(),
        }
    }
}
