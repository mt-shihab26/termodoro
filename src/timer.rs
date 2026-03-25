use crate::config::Config;
use crate::state::SavedState;

#[derive(Debug, Clone, PartialEq)]
pub enum Phase {
    Work,
    Break,
    LongBreak,
}

impl Phase {
    pub fn label(&self) -> &str {
        match self {
            Phase::Work => "Work",
            Phase::Break => "Short Break",
            Phase::LongBreak => "Long Break",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TimerState {
    Running,
    Paused,
}

pub struct Timer {
    pub phase: Phase,
    pub state: TimerState,
    pub remaining_secs: u64,
    pub sessions_completed: u32,
    config: Config,
}

impl Timer {
    pub fn new(config: Config, saved: Option<SavedState>) -> Self {
        let (phase, remaining_secs, sessions_completed) = match saved {
            Some(s) => (s.phase, s.remaining_secs, s.sessions_completed),
            None => (Phase::Work, config.work_session_duration * 60, 0),
        };
        Self {
            phase,
            state: TimerState::Paused,
            remaining_secs,
            sessions_completed,
            config,
        }
    }

    pub fn tick(&mut self) {
        if self.state != TimerState::Running {
            return;
        }
        if self.remaining_secs > 0 {
            self.remaining_secs -= 1;
        } else {
            self.advance();
        }
    }

    pub fn toggle_pause(&mut self) {
        self.state = match self.state {
            TimerState::Running => TimerState::Paused,
            TimerState::Paused => TimerState::Running,
        };
    }

    pub fn skip(&mut self) {
        self.advance();
    }

    pub fn reset(&mut self) {
        self.remaining_secs = self.phase_duration();
        self.state = TimerState::Paused;
    }

    fn advance(&mut self) {
        match self.phase {
            Phase::Work => {
                self.sessions_completed += 1;
                if self.sessions_completed % self.config.long_break_session_interval == 0 {
                    self.phase = Phase::LongBreak;
                    self.remaining_secs = self.config.long_break_session_duration * 60;
                } else {
                    self.phase = Phase::Break;
                    self.remaining_secs = self.config.break_session_duration * 60;
                }
            }
            Phase::Break | Phase::LongBreak => {
                self.phase = Phase::Work;
                self.remaining_secs = self.config.work_session_duration * 60;
            }
        }
        self.state = TimerState::Paused;
    }

    fn phase_duration(&self) -> u64 {
        match self.phase {
            Phase::Work => self.config.work_session_duration * 60,
            Phase::Break => self.config.break_session_duration * 60,
            Phase::LongBreak => self.config.long_break_session_duration * 60,
        }
    }

    pub fn total_secs(&self) -> u64 {
        self.phase_duration()
    }
}
