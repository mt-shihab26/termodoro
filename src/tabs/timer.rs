use std::{
    io::Result,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU8, Ordering},
        mpsc::Sender,
    },
};

use sea_orm::DatabaseConnection;

use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Layout, Rect},
    style::Color,
    widgets::Widget,
};

use crate::{
    caches::timer::TimerCache,
    config::timer::TimerConfig,
    kinds::{event::Event, phase::COLOR},
    log_warn,
    models::{session::Stat, todo::Todo},
    states::timer::TimerState,
    widgets::{
        layout::border::{BorderProps, BorderWidget},
        timer::{
            clock::{ClockProps, ClockWidget},
            hint::{HintProps, HintWidget},
            phase::{PhaseProps, PhaseWidget},
            session::{SessionProps, SessionWidget},
            status::{StatusProps, StatusWidget},
            todo_picker::{TodoPickerAction, TodoPickerState, TodoPickerWidget},
            todo_show::{TodoShowProps, TodoShowWidget},
        },
    },
    workers::timer::spawn,
};

use super::Tab;

/// The timer tab, managing the pomodoro timer UI and its worker thread.
pub struct TimerTab {
    /// Render counter shared with the worker thread to pace `TimerTick` events.
    count: Arc<AtomicU8>,
    /// Shared cache for today's todos and session stats.
    cache: Arc<Mutex<TimerCache>>,
    /// Shared timer state owned by the worker thread.
    state: Arc<Mutex<TimerState>>,
    /// Active todo picker overlay, `None` when closed.
    picker: Option<TodoPickerState>,
}

impl TimerTab {
    /// Creates a new `TimerTab`, spawning the timer worker thread.
    pub fn new(
        sender: Sender<Event>,
        config: TimerConfig,
        cache: Arc<Mutex<TimerCache>>,
        db: DatabaseConnection,
    ) -> Self {
        let count = Arc::new(AtomicU8::new(1));
        let state = spawn(Arc::clone(&count), sender, config, Arc::clone(&cache), db);

        Self {
            count,
            cache,
            state,
            picker: None,
        }
    }

    /// Sets the currently associated todo on the timer state.
    fn set_todo(&mut self, todo_id: Option<i32>) {
        if let Ok(mut s) = self.state.lock() {
            s.todo_id = todo_id;
        }
    }

    /// Confirms the picker selection and closes the overlay.
    fn picker_select(&mut self, id: i32) {
        self.set_todo(Some(id));
        self.picker = None;
    }

    /// Closes the picker without changing the selected todo.
    fn picker_cancel(&mut self) {
        self.picker = None;
    }

    /// Toggles the timer between running and paused.
    fn toggle_running(&self) {
        if let Ok(mut s) = self.state.lock() {
            s.is_running = !s.is_running;
        }
    }

    /// Resets the timer to the full duration of the current phase without advancing.
    fn reset_timer(&self) {
        if let Ok(mut s) = self.state.lock() {
            s.time_millis = s.cycle_phase.duration(&s.config);
            s.is_running = false;
        }
    }

    /// Records the current session and advances to the next phase.
    fn skip_session(&self) {
        if let Ok(mut s) = self.state.lock() {
            s.advance();
        }
    }

    /// Opens the todo picker, loading todos and stats from the cache.
    fn open_picker(&mut self) {
        if let Ok(mut c) = self.cache.lock() {
            let todos = c.get_todos().to_vec();
            let stats = c.get_stats().to_vec();
            self.picker = Some(TodoPickerState::new(todos, stats));
        }
    }

    /// Clears the currently associated todo from the timer state.
    fn clear_todo(&mut self) {
        self.set_todo(None);
    }

    /// Increments the render counter so the worker thread knows a new frame was drawn.
    fn tick_render_count(&self) {
        let current = self.count.load(Ordering::Relaxed);
        let next = (current + 1) % u8::MAX;
        self.count.store(next, Ordering::Relaxed);
    }

    /// Returns the todo and stat for the currently selected todo id, if any.
    fn todo_info(&self) -> (Option<Todo>, Option<Stat>) {
        let Some(id) = self.state.lock().ok().and_then(|s| s.todo_id) else {
            return (None, None);
        };
        let Ok(mut cache) = self.cache.lock() else {
            return (None, None);
        };
        let todo = cache.get_todo(id).cloned();
        let stat = cache.get_stat(id).cloned();
        (todo, stat)
    }
}

impl Tab for TimerTab {
    fn name(&self) -> &str {
        "Timer [^x]"
    }

    fn color(&self) -> Color {
        COLOR
    }

    fn handle(&mut self, key: KeyEvent) -> Result<()> {
        if let Some(picker) = &mut self.picker {
            match picker.handle(key) {
                TodoPickerAction::Select(id) => self.picker_select(id),
                TodoPickerAction::Cancel => self.picker_cancel(),
                TodoPickerAction::None => {}
            }
            return Ok(());
        }

        match key.code {
            KeyCode::Char(' ') => self.toggle_running(),
            KeyCode::Char('r') => self.reset_timer(),
            KeyCode::Char('n') => self.skip_session(),
            KeyCode::Char('t') => self.open_picker(),
            KeyCode::Char('T') => self.clear_todo(),
            _ => {}
        }

        Ok(())
    }

    fn render(&self, frame: &mut Frame, area: Rect) {
        self.tick_render_count();

        let state = match self.state.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                log_warn!("timer state mutex poisoned in render, recovering");
                poisoned.into_inner()
            }
        };

        let color = state.cycle_phase.color();
        let sessions = state.sessions_count;
        let running = state.is_running;
        let long_break_interval = state.config.long_break_interval();
        let phase_label = state.cycle_phase.label().to_string();
        let show_millis = state.config.show_millis();
        let time_millis = state.time_millis;

        drop(state);

        let buf = frame.buffer_mut();

        let inner = BorderWidget::new(&BorderProps::new(self.color()), area).render(area, buf);

        let [session_row, _, phase_row, _, time_row, _, status_row, _, bottom] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(8),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Min(2),
        ])
        .areas(inner);

        SessionWidget::new(&SessionProps::new(sessions, long_break_interval)).render(session_row, buf);
        PhaseWidget::new(&PhaseProps::new(phase_label, color)).render(phase_row, buf);
        ClockWidget::new(&ClockProps::new(show_millis, time_millis, color)).render(time_row, buf);
        StatusWidget::new(&StatusProps::new(running)).render(status_row, buf);

        let [todo_row, hint_row] = Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas(bottom);

        let (todo, stat) = self.todo_info();

        TodoShowWidget::new(&TodoShowProps::new(todo.as_ref(), stat.as_ref())).render(todo_row, buf);
        HintWidget::new(&HintProps::new(self.picker.is_some())).render(hint_row, buf);

        if let Some(picker) = &self.picker {
            TodoPickerWidget::new(&picker.props()).render(inner, buf);
        }
    }
}
