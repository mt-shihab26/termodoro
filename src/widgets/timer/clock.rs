use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::Widget;
use tui_big_text::{BigText, PixelSize};

pub struct ClockWidget {
    pub time: String,
    pub color: Color,
}

impl Widget for &ClockWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        BigText::builder()
            .pixel_size(PixelSize::Full)
            .style(Style::new().fg(self.color).bold())
            .lines(vec![self.time.as_str().into()])
            .centered()
            .build()
            .render(area, buf);
    }
}
