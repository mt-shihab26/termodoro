use std::time::{Duration, Instant};

pub struct Fps {
    pub per_second: f64,
    pub per_lifetime: u64,
    pub visible: bool,
    frame_count_per_second: u32,
    interval_start: Instant,
}

impl Fps {
    pub fn new() -> Self {
        Self {
            per_second: 0.0,
            per_lifetime: 0,
            visible: true,
            frame_count_per_second: 0,
            interval_start: Instant::now() - Duration::from_secs(1),
        }
    }

    pub fn tick(&mut self) {
        self.per_lifetime += 1;
        self.frame_count_per_second += 1;

        let elapsed = self.interval_start.elapsed().as_secs_f64();

        if elapsed >= 1.0 {
            self.per_second = self.frame_count_per_second as f64 / elapsed;
            self.frame_count_per_second = 0;
            self.interval_start = Instant::now();
        }
    }
}
