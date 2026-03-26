use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::Rect};

use crate::ui::app::Shared;
use crate::ui::component::Component;

use super::view::TimerView;

pub struct App;

impl App {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, shared: &mut Shared, frame: &mut Frame, area: Rect) {
        let active = shared.active_todo_label();
        let active_ref = active.as_deref();

        TimerView {
            timer: &shared.timer,
            active_todo: active_ref,
        }
        .render(frame, area);
    }

    pub fn handle_key(&mut self, shared: &mut Shared, key: KeyEvent) {
        match key.code {
            KeyCode::Char(' ') => shared.timer.toggle(),
            KeyCode::Char('s') => {
                let completed = shared.timer.skip();
                shared.record_completed_work(completed);
            }
            KeyCode::Char('r') => shared.timer.reset(),
            KeyCode::Char('u') => shared.clear_active_todo(),
            _ => {}
        }
    }
}
