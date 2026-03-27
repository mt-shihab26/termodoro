use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::domains::timer::{TimerState, tick_interval};
use crate::event::Event;

pub fn spawn(sender: Sender<Event>) -> Arc<Mutex<TimerState>> {
    let state = Arc::new(Mutex::new(TimerState::new()));
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
