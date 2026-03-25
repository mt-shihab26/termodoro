use crate::config::Config;
use crate::state::{self, Phase, State};

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Running,
    Paused,
}

pub struct Timer {
    pub status: Status,
    pub config: Config,
    pub state: State,
}

impl Timer {
    pub fn new(config: Config, saved: Option<State>) -> Self {
        let state = match saved {
            Some(s) => s,
            None => State {
                phase: Phase::Work,
                remaining_secs: config.work_session_duration * 60,
                sessions_completed: 0,
            },
        };

        Self {
            status: Status::Paused,
            config,
            state,
        }
    }

    pub fn toggle(&mut self) {
        self.status = match self.status {
            Status::Running => Status::Paused,
            Status::Paused => Status::Running,
        };

        state::save_state(&self.state);
    }

    pub fn skip(&mut self) {
        self.advance();

        state::save_state(&self.state);
    }

    pub fn reset(&mut self) {
        self.state.remaining_secs = self.phase_duration();
        self.status = Status::Paused;

        state::save_state(&self.state);
    }

    pub fn tick(&mut self) {
        if self.status != Status::Running {
            return;
        }
        if self.state.remaining_secs > 0 {
            self.state.remaining_secs -= 1;
        } else {
            self.advance();
        }
    }

    fn advance(&mut self) {
        match self.state.phase {
            Phase::Work => {
                self.state.sessions_completed += 1;
                if self.state.sessions_completed % self.config.long_break_session_interval == 0 {
                    self.state.phase = Phase::LongBreak;
                    self.state.remaining_secs = self.config.long_break_session_duration * 60;
                } else {
                    self.state.phase = Phase::Break;
                    self.state.remaining_secs = self.config.break_session_duration * 60;
                }
            }
            Phase::Break | Phase::LongBreak => {
                self.state.phase = Phase::Work;
                self.state.remaining_secs = self.config.work_session_duration * 60;
            }
        }
        self.status = Status::Paused;
    }

    fn phase_duration(&self) -> u64 {
        match self.state.phase {
            Phase::Work => self.config.work_session_duration * 60,
            Phase::Break => self.config.break_session_duration * 60,
            Phase::LongBreak => self.config.long_break_session_duration * 60,
        }
    }

    pub fn total_secs(&self) -> u64 {
        self.phase_duration()
    }
}
