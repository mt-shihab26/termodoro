use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::domains::timer::{TimerState, tick_interval};
use crate::event::Event;

pub fn spawn(sender: Sender<Event>, render_count: Arc<AtomicU64>) -> Arc<Mutex<TimerState>> {
    let state = Arc::new(Mutex::new(TimerState::new()));

    let thread_state = Arc::clone(&state);

    thread::spawn(move || {
        let mut last_render_count = u64::MAX;

        loop {
            thread::sleep(Duration::from_millis(tick_interval()));

            let mut state = thread_state.lock().unwrap();
            state.tick();
            let running = state.running;
            drop(state);

            let current_render_count = render_count.load(Ordering::Relaxed);

            if running && current_render_count != last_render_count {
                last_render_count = current_render_count;
                let _ = sender.send(Event::Tick);
            }
        }
    });

    state
}
