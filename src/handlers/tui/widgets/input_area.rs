use ratatui::prelude::{Buffer, Rect, Widget};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, Paragraph};

pub enum InputAreaAction {
    Confirm,
    Cancel,
    None,
}

#[derive(Clone)]
pub struct InputArea {
    text: String,
}

impl InputArea {
    pub fn new() -> Self {
        Self { text: "".to_string() }
    }
}

impl Widget for &InputArea {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(" New Todo ")
            .border_style(Style::default().fg(Color::Cyan));

        let inner = block.inner(area);

        block.render(area, buf);

        Paragraph::new(format!("{}>", self.text))
            .fg(Color::White)
            .render(inner, buf);
    }
}
