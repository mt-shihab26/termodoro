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

    /// Returns the work session duration in milliseconds, clamped to 1–120 minutes.
    pub fn work_duration(&self) -> u32 {
        self.work_duration.clamp(1, 120) * 60 * 1000
    }

    /// Returns the short break duration in milliseconds, clamped to 1–60 minutes.
    pub fn break_duration(&self) -> u32 {
        self.break_duration.clamp(1, 60) * 60 * 1000
    }

    /// Returns the long break duration in milliseconds, clamped to 1–60 minutes.
    pub fn long_break_duration(&self) -> u32 {
        self.long_break_duration.clamp(1, 60) * 60 * 1000
    }

    /// Returns the number of work sessions between long breaks, clamped to 1–10.
    pub fn long_break_interval(&self) -> u32 {
        self.long_break_interval.clamp(1, 10)
    }

    /// Returns the tick interval in milliseconds — 10ms when showing millis, 1000ms otherwise.
    pub fn tick_interval(show_millis: bool) -> u32 {
        if show_millis { 10 } else { 1000 }
    }
}
