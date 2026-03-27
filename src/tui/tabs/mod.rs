pub mod timer;
pub mod todos;

use std::io::Result;

use ratatui::crossterm::event::KeyEvent;

trait Event {
    fn handle(&mut self, key: KeyEvent) -> Result<()>;
}
