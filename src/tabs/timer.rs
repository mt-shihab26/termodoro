use std::{
    io::{Error, ErrorKind, Result},
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
    style::{Color, Stylize},
    widgets::{Block, Widget},
};

use crate::{
    config::timer::TimerConfig,
    kinds::{event::Event, phase::COLOR},
    log_error, log_warn,
    states::{timer::TimerState, timer_cache::TimerCache},
    widgets::timer::{
        clock::{ClockProps, ClockWidget},
        hint::HintWidget,
        phase::{PhaseProps, PhaseWidget},
        session::{SessionProps, SessionWidget},
        status::{StatusProps, StatusWidget},
        todo::{TodoProps, TodoWidget},
        todo_picker::{TodoPickerAction, TodoPickerState, TodoPickerWidget},
    },
    workers::timer::spawn,
};

use super::Tab;

pub struct TimerTab {
    count: Arc<AtomicU8>,
    state: Arc<Mutex<TimerState>>,
    cache: Arc<Mutex<TimerCache>>,
    picker: Option<TodoPickerState>,
    todo: Option<(i32, String)>,
}

impl TimerTab {
    pub fn new(
        sender: Sender<Event>,
        config: TimerConfig,
        cache: Arc<Mutex<TimerCache>>,
        db: DatabaseConnection,
    ) -> Self {
        let count = Arc::new(AtomicU8::new(1));
        let state = spawn(Arc::clone(&count), sender, config, db);

        Self {
            count,
            state,
            cache,
            picker: None,
            todo: None,
        }
    }

    fn tick_render_count(&self) {
        let current = self.count.load(Ordering::Relaxed);
        let next = (current + 1) % u8::MAX;
        self.count.store(next, Ordering::Relaxed);
    }

    fn open_picker(&mut self) {
        let todos = self.cache.lock().map(|mut c| c.todos().to_vec()).unwrap_or_default();
        self.picker = Some(TodoPickerState::new(todos));
    }

    fn set_selected_todo(&mut self, todo: Option<(i32, String)>) {
        let todo_id = todo.as_ref().map(|(id, _)| *id);
        self.todo = todo;
        if let Ok(mut cache) = self.cache.lock() {
            cache.invalidate_stats();
        }
        if let Ok(mut s) = self.state.lock() {
            s.selected_todo_id = todo_id;
        }
    }

    fn refresh_stats_if_needed(&self, sessions: u32) {
        if let Some((todo_id, _)) = &self.todo {
            if let Ok(mut cache) = self.cache.lock() {
                cache.refresh_stats_if_needed(*todo_id, sessions);
            }
        }
    }
}

impl Tab for TimerTab {
    fn name(&self) -> &str {
        "Timer [^2]"
    }

    fn color(&self) -> Color {
        COLOR
    }

    fn handle(&mut self, key: KeyEvent) -> Result<()> {
        if let Some(picker) = &mut self.picker {
            match picker.handle(key) {
                TodoPickerAction::Select(todo) => {
                    self.set_selected_todo(Some(todo));
                    self.picker = None;
                }
                TodoPickerAction::Cancel => {
                    self.picker = None;
                }
                TodoPickerAction::None => {}
            }
            return Ok(());
        }

        let mut s = self.state.lock().map_err(|e| {
            let err = Error::new(ErrorKind::Other, e.to_string());
            log_error!("timer state mutex poisoned in handle: {err}");
            err
        })?;

        match key.code {
            KeyCode::Char(' ') => s.running = !s.running,
            KeyCode::Char('r') => {
                s.time_millis = s.phase.duration(&s.config);
                s.running = false;
            }
            KeyCode::Char('n') => s.advance(false),
            KeyCode::Char('t') => {
                drop(s);
                self.open_picker();
            }
            KeyCode::Char('T') => {
                drop(s);
                self.set_selected_todo(None);
            }
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

        let color = state.phase.color();
        let sessions = state.sessions;
        let running = state.running;
        let long_break_interval = state.config.long_break_interval();
        let phase_label = state.phase.label().to_string();
        let show_millis = state.config.show_millis();
        let time_millis = state.time_millis;

        drop(state);

        self.refresh_stats_if_needed(sessions);

        let todo_text = self.todo.as_ref().map(|(_, t)| t.as_str());
        let todo_stats = self.cache.lock().ok().and_then(|c| c.stats());

        let hint_w = HintWidget {
            selecting_todo: self.picker.is_some(),
        };

        let buf = frame.buffer_mut();

        let block = Block::bordered().fg(self.color());
        let inner = block.inner(area);
        block.render(area, buf);

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

        TodoWidget::new(&TodoProps::new(todo_text, todo_stats)).render(todo_row, buf);

        (&hint_w).render(hint_row, buf);

        if let Some(picker) = &self.picker {
            let props = picker.props();
            TodoPickerWidget::new(&props).render(inner, buf);
        }
    }
}
