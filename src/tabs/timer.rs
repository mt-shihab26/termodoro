use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::widgets::{Block, Paragraph, Widget};

pub struct TimerTab;

impl Widget for TimerTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("In the terminal, we don't just render widgets; we create dreams.")
            .alignment(Alignment::Center)
            .block(Block::bordered())
            .render(area, buf);
    }
}
