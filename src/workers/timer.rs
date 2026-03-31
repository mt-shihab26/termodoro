use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Arc, Mutex, mpsc::Sender};
use std::{thread, time::Duration};

use crate::{domains::timer::state::TimerState, kinds::event::Event, log_error, log_warn};

pub fn spawn(render_count: Arc<AtomicU8>, sender: Sender<Event>) -> Arc<Mutex<TimerState>> {
    let state = Arc::new(Mutex::new(TimerState::new()));

    let thread_state = Arc::clone(&state);

    thread::spawn(move || {
        let mut last_render_count: u8 = u8::MAX;

        loop {
            let (interval, running) = {
                let mut state = match thread_state.lock() {
                    Ok(guard) => guard,
                    Err(poisoned) => {
                        log_warn!("timer state mutex poisoned in worker, recovering");
                        poisoned.into_inner()
                    }
                };

                state.tick();

                (state.config.tick_interval(), state.running)
            };

            thread::sleep(Duration::from_millis(interval));

            let current_render_count = render_count.load(Ordering::Relaxed);

            if running && current_render_count != last_render_count {
                last_render_count = current_render_count;
                if sender.send(Event::Tick).is_err() {
                    log_error!("timer worker: event channel closed, stopping");
                    break;
                }
            }
        }
    });

    state
}
