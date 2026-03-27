use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub const WORK_DURATION: u64 = 25 * 60;
pub const BREAK_DURATION: u64 = 5 * 60;
pub const LONG_BREAK_DURATION: u64 = 15 * 60;
pub const LONG_BREAK_INTERVAL: u32 = 4;

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

pub struct TimerWorker {
    pub phase: Phase,
    pub seconds: u64,
    pub sessions: u32,
    pub running: bool,
}

impl TimerWorker {
    pub fn tick(&mut self) {
        if !self.running {
            return;
        }
        if self.seconds > 0 {
            self.seconds -= 1;
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
        self.seconds = self.phase.duration();
        self.running = false;
    }
}

pub fn spawn(on_tick: impl Fn() + Send + 'static) -> Arc<Mutex<TimerWorker>> {
    let state = Arc::new(Mutex::new(TimerWorker {
        phase: Phase::Work,
        seconds: WORK_DURATION,
        sessions: 0,
        running: false,
    }));

    let thread_state = Arc::clone(&state);

    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
        thread_state.lock().unwrap().tick();
        on_tick();
    });

    state
}
