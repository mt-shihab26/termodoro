use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::{Buffer, Rect, Widget};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, Paragraph};

pub enum InputAreaAction {
    Confirm(String),
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

    pub fn handle(&mut self, key: KeyEvent) -> InputAreaAction {
        match key.code {
            KeyCode::Backspace => {
                self.text.pop();
            }
            KeyCode::Char(c) => {
                self.text.push(c);
            }
            KeyCode::Enter => {
                if !self.text.trim().is_empty() {
                    let text = self.text.clone();
                    self.text.clear();
                    return InputAreaAction::Confirm(text);
                }
            }
            _ => {}
        }
        InputAreaAction::None
    }
}

impl Widget for &InputArea {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(" New Todo ")
            .border_style(Style::default().fg(Color::Cyan));

        let inner = block.inner(area);

        block.render(area, buf);

        Paragraph::new(self.text.clone()).fg(Color::White).render(inner, buf);
    }
}
