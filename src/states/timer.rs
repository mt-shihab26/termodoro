use sea_orm::DatabaseConnection;

use crate::{config::timer::TimerConfig, kinds::phase::Phase, models::session::Session};

pub struct TimerState {
    pub db: DatabaseConnection,
    pub phase: Phase,
    pub time_millis: u32,
    pub sessions: u32,
    pub running: bool,
    pub config: TimerConfig,
    pub todo_id: Option<i32>,
}

impl TimerState {
    pub fn new(timer_config: TimerConfig, db: DatabaseConnection) -> Self {
        Self {
            phase: Phase::Work,
            time_millis: timer_config.work_duration(),
            sessions: 0,
            running: false,
            config: timer_config,
            db,
            todo_id: None,
        }
    }

    pub fn tick(&mut self) {
        if !self.running {
            return;
        }

        let step = self.config.tick_interval();

        if self.time_millis >= step {
            self.time_millis -= step;
        } else {
            self.advance(true);
        }
    }

    pub fn advance(&mut self, completed: bool) {
        let duration = self.phase.duration(&self.config);
        Session::record(&self.db, &self.phase, duration, self.todo_id, completed);

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
        self.time_millis = self.phase.duration(&self.config);
        self.running = false;
    }
}
