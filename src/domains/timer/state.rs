use crate::kinds::phase::Phase;

use super::config::Config;

pub struct TimerState {
    pub phase: Phase,
    pub millis: u64,
    pub sessions: u64,
    pub running: bool,
    pub config: Config,
}

impl TimerState {
    pub fn new() -> Self {
        let config = Config::new();

        Self {
            phase: Phase::Work,
            millis: config.work_duration(),
            sessions: 0,
            running: false,
            config: Config::new(),
        }
    }

    pub fn tick(&mut self) {
        if !self.running {
            return;
        }

        let step = self.config.tick_interval();

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
                self.phase = if self.sessions % self.config.long_break_interval() == 0 {
                    Phase::LongBreak
                } else {
                    Phase::Break
                };
            }
            Phase::Break | Phase::LongBreak => {
                self.phase = Phase::Work;
            }
        }
        self.millis = self.phase.duration(&self.config);
        self.running = false;
    }

    pub fn time_parts(&self) -> (u64, u64, u64) {
        let mins = self.millis / 60000;
        let secs = (self.millis / 1000) % 60;
        let cs = (self.millis % 1000) / 10;
        (mins, secs, cs)
    }
}
