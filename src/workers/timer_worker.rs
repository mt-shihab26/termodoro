use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::event::Event;

pub const WORK_DURATION: u64 = 25 * 60 * 1000;
pub const BREAK_DURATION: u64 = 5 * 60 * 1000;
pub const LONG_BREAK_DURATION: u64 = 15 * 60 * 1000;
pub const LONG_BREAK_INTERVAL: u32 = 4;
pub const SHOW_MILLIS: bool = false;

fn tick_interval() -> u64 {
    if SHOW_MILLIS { 10 } else { 1000 }
}

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

pub struct TimerState {
    pub phase: Phase,
    pub millis: u64,
    pub sessions: u32,
    pub running: bool,
}

impl TimerState {
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

pub fn spawn(sender: Sender<Event>) -> Arc<Mutex<TimerState>> {
    let state = Arc::new(Mutex::new(TimerState {
        phase: Phase::Work,
        millis: WORK_DURATION,
        sessions: 0,
        running: false,
    }));

    let thread_state = Arc::clone(&state);

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(tick_interval()));
            let mut state = thread_state.lock().unwrap();
            state.tick();
            let running = state.running;
            drop(state);
            if running {
                let _ = sender.send(Event::Tick);
            }
        }
    });

    state
}
