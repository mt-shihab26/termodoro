pub mod timer;
pub mod todos;

use std::io::Result;

use ratatui::crossterm::event::KeyEvent;
use ratatui::style::Color;
use ratatui::{Frame, layout::Rect};

pub trait Tab {
    fn name(&self) -> &str;
    fn color(&self) -> Color;
    fn render(&self, frame: &mut Frame, area: Rect);
    fn handle(&mut self, key: KeyEvent) -> Result<()>;
    fn tick(&mut self) {}
}
