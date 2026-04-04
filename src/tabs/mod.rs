/// Timer tab implementation for pomodoro controls and rendering.
pub mod timer;
/// Todos tab implementation for task list navigation and editing.
pub mod todos;

use std::io::Result;

use ratatui::{Frame, crossterm::event::KeyEvent, layout::Rect, style::Color};

/// Common interface implemented by every top-level tab.
pub trait Tab {
    /// Returns the display name shown in the tab bar.
    fn name(&self) -> &str;
    /// Returns the accent color used for this tab's UI elements.
    fn color(&self) -> Color;
    /// Handles a key event and updates internal state accordingly.
    fn handle(&mut self, key: KeyEvent) -> Result<()>;
    /// Renders the tab contents into the given frame area.
    fn render(&self, frame: &mut Frame, area: Rect);
    /// Returns `true` if this tab needs periodic tick events; defaults to `false`.
    fn should_tick(&self) -> bool {
        false
    }
    /// Called on each tick when `should_tick` returns `true`; defaults to no-op.
    fn next_tick(&mut self) -> Result<()> {
        Ok(())
    }
}
