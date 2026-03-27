use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::widgets::{Block, Paragraph, Widget};

pub struct TodosTab;

impl Widget for TodosTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Great terminal interfaces start with a single widget.")
            .alignment(Alignment::Center)
            .block(Block::bordered())
            .render(area, buf);
    }
}
