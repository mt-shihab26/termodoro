use std::time::{Duration, Instant};

use ratatui::prelude::{Buffer, Rect};
use ratatui::style::{Color, Stylize};
use ratatui::text::{Line, Span};
use ratatui::{layout::Alignment, widgets::Widget};

pub struct FpsWidget {
    pub per_second: f64,
    pub per_lifetime: u64,
    frame_count_per_second: u32,
    interval_start: Instant,
}

impl FpsWidget {
    pub fn new() -> Self {
        Self {
            per_second: 0.0,
            per_lifetime: 0,
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

impl Widget for &FpsWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Line::from(
            Span::from(format!(
                "{:.0} fps  {} frames",
                self.per_second, self.per_lifetime
            ))
            .fg(Color::DarkGray),
        )
        .alignment(Alignment::Right)
        .render(area, buf);
    }
}
