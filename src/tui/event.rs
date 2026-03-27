use ratatui::crossterm::event::KeyEvent;

pub enum AppEvent {
    Key(KeyEvent),
    Tick,
}
