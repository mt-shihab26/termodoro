use ratatui::crossterm::event::Event;

pub enum AppEvent {
    Term(Event),
    Tick,
}
