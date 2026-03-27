pub const WORK_DURATION: u64 = 25 * 60 * 1000;
pub const BREAK_DURATION: u64 = 5 * 60 * 1000;
pub const LONG_BREAK_DURATION: u64 = 15 * 60 * 1000;
pub const LONG_BREAK_INTERVAL: u32 = 4;
pub const SHOW_MILLIS: bool = true;

pub fn tick_interval() -> u64 {
    if SHOW_MILLIS { 10 } else { 1000 }
}
