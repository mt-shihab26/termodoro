use ratatui::crossterm::event::KeyEvent;

pub enum Event {
    Key(KeyEvent),
    Resize(u16, u16),
    TimerTick,
    NavigationTick,
}
