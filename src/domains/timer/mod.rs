mod config;
mod phase;
mod state;

pub use config::{LONG_BREAK_INTERVAL, SHOW_MILLIS, tick_interval};
pub use phase::Phase;
pub use state::TimerState;
