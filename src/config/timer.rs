use serde::{Deserialize, Serialize};

// TODO: add docs

#[derive(Debug, Deserialize, Serialize)]
pub struct TimerConfig {
    #[serde(default)]
    show_millis: bool,
    #[serde(default)]
    work_duration: u32,
    #[serde(default)]
    break_duration: u32,
    #[serde(default)]
    long_break_duration: u32,
    #[serde(default)]
    long_break_interval: u32,
}

impl Default for TimerConfig {
    fn default() -> Self {
        Self {
            show_millis: false,
            work_duration: 25,
            break_duration: 5,
            long_break_duration: 15,
            long_break_interval: 4,
        }
    }
}

impl TimerConfig {
    pub fn show_millis(&self) -> bool {
        self.show_millis
    }

    pub fn work_duration(&self) -> u32 {
        self.work_duration as u32 * 60 * 1000
    }

    pub fn break_duration(&self) -> u32 {
        self.break_duration as u32 * 60 * 1000
    }

    pub fn long_break_duration(&self) -> u32 {
        self.long_break_duration as u32 * 60 * 1000
    }

    pub fn long_break_interval(&self) -> u32 {
        self.long_break_interval
    }

    pub fn tick_interval(&self) -> u32 {
        if self.show_millis { 10 } else { 1000 }
    }
}
