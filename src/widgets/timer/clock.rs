use ratatui::prelude::{Buffer, Color, Rect, Style, Widget};
use tui_big_text::{BigText, PixelSize};

/// Props for the big-text countdown clock.
pub struct ClockProps {
    /// Whether to display centiseconds alongside mm:ss.
    show_millis: bool,
    /// Current time value in milliseconds.
    time_millis: u32,
    /// Foreground color used to render the clock digits.
    color: Color,
}

impl ClockProps {
    /// Creates new clock props.
    pub fn new(show_millis: bool, time_millis: u32, color: Color) -> Self {
        Self {
            show_millis,
            time_millis,
            color,
        }
    }
}

/// Stateless widget that renders a big-text clock.
pub struct ClockWidget<'a> {
    /// Borrowed clock props for this render pass.
    props: &'a ClockProps,
}

impl<'a> ClockWidget<'a> {
    /// Creates a new clock widget from the given props.
    pub fn new(props: &'a ClockProps) -> Self {
        Self { props }
    }

    /// Returns the time as a formatted string (mm:ss or mm:ss.cs).
    fn formatted_time(&'a self) -> String {
        let (mins, secs, ms) = self.time_parts();

        if self.props.show_millis {
            format!("{:02}:{:02}.{:02}", mins, secs, ms)
        } else {
            format!("{:02}:{:02}", mins, secs)
        }
    }

    /// Splits the millisecond value into (minutes, seconds, centiseconds).
    fn time_parts(&'a self) -> (u32, u32, u32) {
        let mins = self.props.time_millis / 60000;
        let secs = (self.props.time_millis / 1000) % 60;
        let cs = (self.props.time_millis % 1000) / 10;
        (mins, secs, cs)
    }
}

impl Widget for &ClockWidget<'_> {
    /// Renders the clock into the given buffer area.
    fn render(self, area: Rect, buf: &mut Buffer) {
        BigText::builder()
            .pixel_size(PixelSize::Full)
            .style(Style::new().fg(self.props.color).bold())
            .lines(vec![self.formatted_time().as_str().into()])
            .centered()
            .build()
            .render(area, buf);
    }
}
