use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TimerConfig {
    #[serde(default)]
    pub show_millis: bool,
    #[serde(default)]
    pub work_duration: u64,
    #[serde(default)]
    pub break_duration: u64,
    #[serde(default)]
    pub long_break_duration: u64,
    #[serde(default)]
    pub long_break_interval: u64,
}

impl Default for TimerConfig {
    fn default() -> Self {
        Self {
            show_millis: true,
            work_duration: 25,
            break_duration: 5,
            long_break_duration: 15,
            long_break_interval: 4,
        }
    }
}
