use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Arc, Mutex, mpsc::Sender};
use std::{thread, time::Duration};

use sea_orm::DatabaseConnection;

use crate::config::timer::TimerConfig;
use crate::states::timer::TimerState;
use crate::{kinds::event::Event, log_error, log_warn};

pub fn spawn(
    render_count: Arc<AtomicU8>,
    sender: Sender<Event>,
    timer_config: TimerConfig,
    db: DatabaseConnection,
) -> Arc<Mutex<TimerState>> {
    let state = Arc::new(Mutex::new(TimerState::new(timer_config, db)));

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

                (state.timer_config.tick_interval(), state.running)
            };

            thread::sleep(Duration::from_millis(interval as u64));

            let current_render_count = render_count.load(Ordering::Relaxed);

            if running && current_render_count != last_render_count {
                last_render_count = current_render_count;
                if sender.send(Event::TimerTick).is_err() {
                    log_error!("timer worker: event channel closed, stopping");
                    break;
                }
            }
        }
    });

    state
}
