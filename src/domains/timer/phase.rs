use super::config::{BREAK_DURATION, LONG_BREAK_DURATION, WORK_DURATION};

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

    pub fn duration(&self) -> u64 {
        match self {
            Phase::Work => WORK_DURATION,
            Phase::Break => BREAK_DURATION,
            Phase::LongBreak => LONG_BREAK_DURATION,
        }
    }
}
