use std::time::{Duration, Instant};

use ratatui::prelude::{Alignment, Buffer, Color, Line, Rect, Span, Stylize, Widget};

// Props — data snapshot, borrowed by the widget at render time.
pub struct FpsProps {
    pub per_second: f64,
    pub per_lifetime: u64,
}

impl FpsProps {
    pub fn new(per_second: f64, per_lifetime: u64) -> Self {
        Self {
            per_second,
            per_lifetime,
        }
    }
}

// State — owned by App, updated each frame via tick().
// Exposes `props` so App can pass &state.props to the widget.
pub struct FpsState {
    pub props: FpsProps,
    frame_count_per_second: u32,
    interval_start: Instant,
}

impl FpsState {
    pub fn new() -> Self {
        Self {
            props: FpsProps::new(0.0, 0),
            frame_count_per_second: 0,
            interval_start: Instant::now() - Duration::from_secs(1),
        }
    }

    /// Call once per frame before rendering.
    pub fn tick(&mut self) {
        self.props.per_lifetime += 1;
        self.frame_count_per_second += 1;

        let elapsed = self.interval_start.elapsed().as_secs_f64();

        if elapsed >= 1.0 {
            self.props.per_second = self.frame_count_per_second as f64 / elapsed;
            self.frame_count_per_second = 0;
            self.interval_start = Instant::now();
        }
    }
}

// Widget — stateless view, borrows Props for one render call.
pub struct FpsWidget<'a> {
    props: &'a FpsProps,
}

impl<'a> FpsWidget<'a> {
    pub fn new(props: &'a FpsProps) -> Self {
        Self { props }
    }
}

impl<'a> Widget for &FpsWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Line::from(
            Span::from(format!(
                "{:.0} fps  {} frames",
                self.props.per_second, self.props.per_lifetime
            ))
            .fg(Color::DarkGray),
        )
        .alignment(Alignment::Right)
        .render(area, buf);
    }
}
