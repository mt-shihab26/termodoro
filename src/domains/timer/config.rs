pub struct Config {
    show_millis: bool,
    work_duration: u64,
    break_duration: u64,
    long_break_duration: u64,
    long_break_interval: u64,
}

impl Config {
    pub fn new() -> Self {
        Self {
            show_millis: true,
            work_duration: 25,
            break_duration: 5,
            long_break_duration: 15,
            long_break_interval: 4,
        }
    }

    pub fn show_millis(&self) -> bool {
        self.show_millis
    }

    pub fn work_duration(&self) -> u64 {
        self.work_duration * 60 * 1000
    }

    pub fn break_duration(&self) -> u64 {
        self.break_duration * 60 * 1000
    }

    pub fn long_break_duration(&self) -> u64 {
        self.long_break_duration * 60 * 1000
    }

    pub fn long_break_interval(&self) -> u64 {
        self.long_break_interval
    }

    pub fn tick_interval(&self) -> u64 {
        if self.show_millis { 10 } else { 1000 }
    }
}
