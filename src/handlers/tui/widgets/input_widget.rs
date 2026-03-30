use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::{Buffer, Rect, Widget};
use ratatui::style::Style;
use ratatui::widgets::Block;
use ratatui_textarea::TextArea;

use crate::handlers::tui::tabs::todos::COLOR;

pub enum InputAreaAction {
    Confirm(String),
    Escape,
    None,
}

pub struct InputArea {
    textarea: TextArea<'static>,
}

impl InputArea {
    pub fn new(text: Option<&str>) -> Self {
        let mut textarea = TextArea::default();
        if let Some(t) = text {
            textarea.insert_str(t);
        }
        textarea.set_block(Block::bordered().border_style(Style::default().fg(COLOR)));
        textarea.set_cursor_line_style(Style::default());
        Self { textarea }
    }

    pub fn handle(&mut self, key: KeyEvent) -> InputAreaAction {
        match key.code {
            KeyCode::Enter => {
                let text = self.textarea.lines()[0].clone();
                if !text.trim().is_empty() {
                    return InputAreaAction::Confirm(text);
                }
            }
            KeyCode::Esc => return InputAreaAction::Escape,
            _ => {
                self.textarea.input(key);
            }
        }
        InputAreaAction::None
    }
}

impl Widget for &InputArea {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Widget::render(&self.textarea, area, buf);
    }
}
