pub mod timer;
pub mod todos;

use std::io::Result;

use ratatui::{Frame, crossterm::event::KeyEvent, layout::Rect, style::Color};

pub trait Tab {
    fn name(&self) -> &str;
    fn color(&self) -> Color;
    fn render(&self, frame: &mut Frame, area: Rect);
    fn handle(&mut self, key: KeyEvent) -> Result<()>;
}
