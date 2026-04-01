use sea_orm::DatabaseConnection;

use crate::{config::timer::TimerConfig, kinds::phase::Phase, models::session::Session};

pub struct TimerState {
    pub db: DatabaseConnection,
    pub phase: Phase,
    pub millis: u32,
    pub sessions: u32,
    pub running: bool,
    pub timer_config: TimerConfig,
    pub selected_todo_id: Option<i32>,
}

impl TimerState {
    pub fn new(timer_config: TimerConfig, db: DatabaseConnection) -> Self {
        Self {
            phase: Phase::Work,
            millis: timer_config.work_duration(),
            sessions: 0,
            running: false,
            timer_config,
            db,
            selected_todo_id: None,
        }
    }

    pub fn tick(&mut self) {
        if !self.running {
            return;
        }

        let step = self.timer_config.tick_interval();

        if self.millis >= step {
            self.millis -= step;
        } else {
            self.advance();
        }
    }

    pub fn advance(&mut self) {
        let duration = self.phase.duration(&self.timer_config);
        Session::record(&self.db, &self.phase, duration, self.selected_todo_id);

        match self.phase {
            Phase::Work => {
                self.sessions += 1;
                self.phase = if self.sessions % self.timer_config.long_break_interval() == 0 {
                    Phase::LongBreak
                } else {
                    Phase::Break
                };
            }
            Phase::Break | Phase::LongBreak => {
                self.phase = Phase::Work;
            }
        }
        self.millis = self.phase.duration(&self.timer_config);
        self.running = false;
    }

    pub fn time_parts(&self) -> (u32, u32, u32) {
        let mins = self.millis / 60000;
        let secs = (self.millis / 1000) % 60;
        let cs = (self.millis % 1000) / 10;
        (mins, secs, cs)
    }
}
