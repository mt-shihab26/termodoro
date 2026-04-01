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
    caches::timer::{Stat, TimerCache},
    config::timer::TimerConfig,
    kinds::{event::Event, phase::COLOR},
    log_warn,
    models::todo::Todo,
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

pub struct TimerTab {
    count: Arc<AtomicU8>,
    state: Arc<Mutex<TimerState>>,
    cache: Arc<Mutex<TimerCache>>,
    picker: Option<TodoPickerState>,
    todo_id: Option<i32>,
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
            todo_id: None,
        }
    }

    fn tick_render_count(&self) {
        let current = self.count.load(Ordering::Relaxed);
        let next = (current + 1) % u8::MAX;
        self.count.store(next, Ordering::Relaxed);
    }

    fn open_picker(&mut self) {
        let todos = self
            .cache
            .lock()
            .map(|mut c| {
                let stats = c.get_stats().to_vec();
                c.get_todos().iter().zip(stats).map(|(t, s)| (t.clone(), s)).collect()
            })
            .unwrap_or_default();
        self.picker = Some(TodoPickerState::new(todos));
    }

    fn set_selected_todo(&mut self, id: Option<i32>) {
        self.todo_id = id;
        if let Ok(mut cache) = self.cache.lock() {
            cache.invalidate_stats();
        }
        if let Ok(mut s) = self.state.lock() {
            s.selected_todo_id = self.todo_id;
        }
    }

    fn toggle_running(&self) {
        if let Ok(mut s) = self.state.lock() {
            s.running = !s.running;
        }
    }

    fn reset(&self) {
        if let Ok(mut s) = self.state.lock() {
            s.time_millis = s.phase.duration(&s.config);
            s.running = false;
        }
    }

    fn skip(&self) {
        if let Ok(mut s) = self.state.lock() {
            s.advance(false);
        }
    }

    fn on_picker_select(&mut self, id: i32) {
        self.set_selected_todo(Some(id));
        self.picker = None;
    }

    fn on_picker_cancel(&mut self) {
        self.picker = None;
    }

    fn todo_info(&self) -> (Option<Todo>, Option<Stat>) {
        let Some(id) = self.todo_id else {
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
        "Timer [^2]"
    }

    fn color(&self) -> Color {
        COLOR
    }

    fn handle(&mut self, key: KeyEvent) -> Result<()> {
        if let Some(picker) = &mut self.picker {
            match picker.handle(key) {
                TodoPickerAction::Select(id) => self.on_picker_select(id),
                TodoPickerAction::Cancel => self.on_picker_cancel(),
                TodoPickerAction::None => {}
            }
            return Ok(());
        }

        match key.code {
            KeyCode::Char(' ') => self.toggle_running(),
            KeyCode::Char('r') => self.reset(),
            KeyCode::Char('n') => self.skip(),
            KeyCode::Char('t') => self.open_picker(),
            KeyCode::Char('T') => self.set_selected_todo(None),
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
