use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::{Buffer, Rect, Widget};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, Paragraph};

use crate::handlers::tui::tabs::todos::COLOR;

pub enum InputAreaAction {
    Confirm(String),
    None,
}

#[derive(Clone)]
pub struct InputArea {
    text: String,
}

impl InputArea {
    pub fn new(text: Option<&str>) -> Self {
        Self {
            text: text.unwrap_or("").to_string(),
        }
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
            .border_style(Style::default().fg(COLOR));

        let inner = block.inner(area);

        block.render(area, buf);

        let display = format!("{}█", self.text);
        Paragraph::new(display).fg(Color::White).render(inner, buf);
    }
}
