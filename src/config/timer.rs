use serde::{Deserialize, Serialize};

/// Configuration for the pomodoro timer, loaded from the user's config file.
#[derive(Debug, Deserialize, Serialize)]
pub struct TimerConfig {
    /// Whether to display milliseconds on the clock.
    #[serde(default)]
    show_millis: bool,
    /// Work session duration in minutes.
    #[serde(default)]
    work_duration: u32,
    /// Short break duration in minutes.
    #[serde(default)]
    break_duration: u32,
    /// Long break duration in minutes.
    #[serde(default)]
    long_break_duration: u32,
    /// Number of work sessions before a long break.
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
    /// Returns whether milliseconds should be shown on the clock.
    pub fn show_millis(&self) -> bool {
        self.show_millis
    }

    /// Returns the work session duration in milliseconds.
    pub fn work_duration(&self) -> u32 {
        self.work_duration as u32 * 60 * 1000
    }

    /// Returns the short break duration in milliseconds.
    pub fn break_duration(&self) -> u32 {
        self.break_duration as u32 * 60 * 1000
    }

    /// Returns the long break duration in milliseconds.
    pub fn long_break_duration(&self) -> u32 {
        self.long_break_duration as u32 * 60 * 1000
    }

    /// Returns the number of work sessions between long breaks.
    pub fn long_break_interval(&self) -> u32 {
        self.long_break_interval
    }

    /// Returns the tick interval in milliseconds — 10ms when showing millis, 1000ms otherwise.
    pub fn tick_interval(&self) -> u32 {
        if self.show_millis { 10 } else { 1000 }
    }
}
