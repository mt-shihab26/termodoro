use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicU8, Ordering},
        mpsc::Sender,
    },
    thread,
    time::Duration,
};

use sea_orm::DatabaseConnection;

use crate::{
    caches::timer::TimerCache, config::timer::TimerConfig, kinds::event::Event, log_error, log_warn,
    states::timer::TimerState, utils::store::Store,
};

/// Spawns the timer worker thread and returns a shared handle to its state.
///
/// The worker ticks the timer at the configured interval and sends a `TimerTick`
/// event whenever the UI has rendered a new frame (detected via `count`), ensuring
/// the display stays in sync without flooding the event queue.
pub fn spawn(
    count: Arc<AtomicU8>,
    sender: Sender<Event>,
    db: DatabaseConnection,
    config: TimerConfig,
    cache: Arc<Mutex<TimerCache>>,
    store: Store,
) -> Arc<Mutex<TimerState>> {
    let state = Arc::new(Mutex::new(TimerState::new(db, config, cache, store)));

    let thread_state = Arc::clone(&state);

    thread::spawn(move || {
        let mut last_render_count: u8 = u8::MAX;
        let mut last_save = std::time::Instant::now();

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

                if last_save.elapsed() >= Duration::from_secs(60) {
                    state.save_remaining();
                    last_save = std::time::Instant::now();
                }

                (TimerConfig::tick_interval(state.show_millis()), state.is_running())
            };

            thread::sleep(Duration::from_millis(interval as u64));

            let current_render_count = count.load(Ordering::Relaxed);

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
