use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TimerConfig {
    #[serde(default = "TimerConfig::default_show_millis")]
    pub show_millis: bool,
    #[serde(default = "TimerConfig::default_work_duration")]
    pub work_duration: u64,
    #[serde(default = "TimerConfig::default_break_duration")]
    pub break_duration: u64,
    #[serde(default = "TimerConfig::default_long_break_duration")]
    pub long_break_duration: u64,
    #[serde(default = "TimerConfig::default_long_break_interval")]
    pub long_break_interval: u64,
}

impl TimerConfig {
    fn default_show_millis() -> bool {
        true
    }
    fn default_work_duration() -> u64 {
        25
    }
    fn default_break_duration() -> u64 {
        5
    }
    fn default_long_break_duration() -> u64 {
        15
    }
    fn default_long_break_interval() -> u64 {
        4
    }
}

impl Default for TimerConfig {
    fn default() -> Self {
        Self {
            show_millis: Self::default_show_millis(),
            work_duration: Self::default_work_duration(),
            break_duration: Self::default_break_duration(),
            long_break_duration: Self::default_long_break_duration(),
            long_break_interval: Self::default_long_break_interval(),
        }
    }
}
