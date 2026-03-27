use super::config::{LONG_BREAK_INTERVAL, WORK_DURATION, tick_interval};
use super::phase::Phase;

pub struct TimerState {
    pub phase: Phase,
    pub millis: u64,
    pub sessions: u32,
    pub running: bool,
}

impl TimerState {
    pub fn new() -> Self {
        Self {
            phase: Phase::Work,
            millis: WORK_DURATION,
            sessions: 0,
            running: false,
        }
    }

    pub fn tick(&mut self) {
        if !self.running {
            return;
        }

        let step = tick_interval();

        if self.millis >= step {
            self.millis -= step;
        } else {
            self.advance();
        }
    }

    pub fn advance(&mut self) {
        match self.phase {
            Phase::Work => {
                self.sessions += 1;
                self.phase = if self.sessions % LONG_BREAK_INTERVAL == 0 {
                    Phase::LongBreak
                } else {
                    Phase::Break
                };
            }
            Phase::Break | Phase::LongBreak => {
                self.phase = Phase::Work;
            }
        }
        self.millis = self.phase.duration();
        self.running = false;
    }
}
