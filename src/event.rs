use ratatui::crossterm::event::KeyEvent;

pub enum Event {
    Key(KeyEvent),
    Tick,
}
