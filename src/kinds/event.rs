//! Event types sent through the application event loop.

use ratatui::crossterm::event::KeyEvent;

/// Application event variants produced by input handling and background work.
pub enum Event {
    /// A keyboard input event from crossterm.
    Key(KeyEvent),
    /// A terminal resize event carrying the new width and height.
    Resize(u16, u16),
    /// A periodic timer tick emitted by the timer worker.
    TimerTick,
}
