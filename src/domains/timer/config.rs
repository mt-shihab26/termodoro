use crate::config::timer::TimerConfig;

pub struct Config {
    inner: TimerConfig,
}

impl Config {
    pub fn new(timer: TimerConfig) -> Self {
        Self { inner: timer }
    }

    pub fn show_millis(&self) -> bool {
        self.inner.show_millis()
    }

    pub fn work_duration(&self) -> u64 {
        self.inner.work_duration()
    }

    pub fn break_duration(&self) -> u64 {
        self.inner.break_duration()
    }

    pub fn long_break_duration(&self) -> u64 {
        self.inner.long_break_duration()
    }

    pub fn long_break_interval(&self) -> u16 {
        self.inner.long_break_interval()
    }

    pub fn tick_interval(&self) -> u64 {
        self.inner.tick_interval()
    }
}
