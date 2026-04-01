use std::sync::{Arc, Mutex};

use sea_orm::DatabaseConnection;

use crate::{caches::timer::TimerCache, config::timer::TimerConfig, kinds::phase::Phase, models::session::Session};

/// Runtime state for the pomodoro timer, owned by the timer worker thread.
pub struct TimerState {
    pub db: DatabaseConnection,
    pub config: TimerConfig,
    pub cache: Arc<Mutex<TimerCache>>,
    /// Whether the timer is actively counting down.
    pub is_running: bool,
    /// Remaining time in the current phase, in milliseconds.
    pub time_millis: u32,
    /// Current phase of the pomodoro cycle (work, break, or long break).
    pub cycle_phase: Phase,
    /// Number of completed work sessions since the timer started.
    pub sessions_count: u32,
    /// The currently selected todo id, used to associate sessions.
    pub todo_id: Option<i32>,
}

impl TimerState {
    pub fn new(config: TimerConfig, cache: Arc<Mutex<TimerCache>>, db: DatabaseConnection) -> Self {
        Self {
            cycle_phase: Phase::Work,
            time_millis: config.work_duration(),
            sessions_count: 0,
            is_running: false,
            config,
            db,
            todo_id: None,
            cache,
        }
    }

    /// Called every tick interval — decrements the timer or advances the phase when it expires.
    pub fn tick(&mut self) {
        if !self.is_running {
            return;
        }

        let step = self.config.tick_interval();

        if self.time_millis >= step {
            self.time_millis -= step;
        } else {
            self.advance();
        }
    }

    /// Records the current session and moves to the next phase.
    pub fn advance(&mut self) {
        let duration = self.cycle_phase.duration(&self.config);

        Session::record(&self.db, &self.cycle_phase, duration, self.todo_id);

        match self.cycle_phase {
            Phase::Work => {
                self.sessions_count += 1;
                self.cycle_phase = if self.sessions_count % self.config.long_break_interval() == 0 {
                    Phase::LongBreak
                } else {
                    Phase::Break
                };
            }
            Phase::Break | Phase::LongBreak => {
                self.cycle_phase = Phase::Work;
            }
        }
        self.time_millis = self.cycle_phase.duration(&self.config);
        self.is_running = false;

        if let Ok(mut c) = self.cache.lock() {
            c.invalidate_stats();
        }
    }
}
