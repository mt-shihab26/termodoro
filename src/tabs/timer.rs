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
    kinds::{event::Event, phase::COLOR, timer_mode::TimerMode},
    log_error, log_warn,
    states::{timer::TimerState, timer_cache::TimerCache},
    widgets::timer::{
        clock::{ClockProps, ClockWidget},
        hint::HintWidget,
        phase::{PhaseProps, PhaseWidget},
        session::{SessionProps, SessionWidget},
        status::StatusWidget,
        todo::TodoWidget,
        todo_picker::TodoPickerWidget,
    },
    workers::timer::spawn,
};

use super::Tab;

pub struct TimerTab {
    count: Arc<AtomicU8>,
    mode: TimerMode,
    state: Arc<Mutex<TimerState>>,
    cache: Arc<Mutex<TimerCache>>,
    // ---
    todos: Vec<(i32, String)>,
    cursor: usize,
    selected: Option<(i32, String)>,
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
            mode: TimerMode::Normal,
            state,
            cache,
            todos: vec![],
            cursor: 0,
            selected: None,
        }
    }

    fn tick_render_count(&self) {
        let current = self.count.load(Ordering::Relaxed);
        let next = (current + 1) % u8::MAX;
        self.count.store(next, Ordering::Relaxed);
    }

    fn load_todos(&mut self) {
        if let Ok(mut cache) = self.cache.lock() {
            self.todos = cache.todos().to_vec();
        }
        if !self.todos.is_empty() && self.cursor >= self.todos.len() {
            self.cursor = self.todos.len() - 1;
        }
    }

    fn set_selected_todo(&mut self, todo: Option<(i32, String)>) {
        let todo_id = todo.as_ref().map(|(id, _)| *id);
        self.selected = todo;
        if let Ok(mut cache) = self.cache.lock() {
            cache.invalidate_stats();
        }
        if let Ok(mut s) = self.state.lock() {
            s.selected_todo_id = todo_id;
        }
    }

    fn refresh_stats_if_needed(&self, sessions: u32) {
        if let Some((todo_id, _)) = &self.selected {
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
        match self.mode {
            TimerMode::Normal => {
                let mut s = self.state.lock().map_err(|e| {
                    let err = Error::new(ErrorKind::Other, e.to_string());
                    log_error!("timer state mutex poisoned in handle: {err}");
                    err
                })?;

                match key.code {
                    KeyCode::Char(' ') => {
                        s.running = !s.running;
                    }
                    KeyCode::Char('r') => {
                        s.time_millis = s.phase.duration(&s.config);
                        s.running = false;
                    }
                    KeyCode::Char('n') => {
                        s.advance(false);
                    }
                    KeyCode::Char('t') => {
                        drop(s);
                        self.load_todos();
                        self.mode = TimerMode::SelectingTodo;
                        return Ok(());
                    }
                    KeyCode::Char('T') => {
                        drop(s);
                        self.set_selected_todo(None);
                        return Ok(());
                    }
                    _ => {}
                }
            }
            TimerMode::SelectingTodo => match key.code {
                KeyCode::Char('j') | KeyCode::Down => {
                    if !self.todos.is_empty() {
                        self.cursor = (self.cursor + 1).min(self.todos.len() - 1);
                    }
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    self.cursor = self.cursor.saturating_sub(1);
                }
                KeyCode::Enter => {
                    if let Some(todo) = self.todos.get(self.cursor).cloned() {
                        self.set_selected_todo(Some(todo));
                    }
                    self.mode = TimerMode::Normal;
                }
                KeyCode::Esc => {
                    self.mode = TimerMode::Normal;
                }
                _ => {}
            },
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

        let status_w = StatusWidget { running };
        let hint_w = HintWidget {
            selecting_todo: matches!(self.mode, TimerMode::SelectingTodo),
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

        (&status_w).render(status_row, buf);

        match self.mode {
            TimerMode::Normal => {
                let [todo_row, hint_row] =
                    Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas(bottom);

                let stats = self.cache.lock().ok().and_then(|c| c.stats());
                let todo_w = TodoWidget {
                    selected: self.selected.as_ref().map(|(_, t)| t.as_str()),
                    stats,
                };

                (&todo_w).render(todo_row, buf);
                (&hint_w).render(hint_row, buf);
            }

            TimerMode::SelectingTodo => {
                let list_height = (self.todos.len().min(5) as u16).max(1);
                let [picker_header, picker_list, hint_row] = Layout::vertical([
                    Constraint::Length(1),
                    Constraint::Length(list_height),
                    Constraint::Length(1),
                ])
                .areas(bottom);

                let picker_w = TodoPickerWidget {
                    todos: &self.todos,
                    cursor: self.cursor,
                };

                ratatui::widgets::Paragraph::new("Select a todo")
                    .centered()
                    .bold()
                    .fg(Color::Yellow)
                    .render(picker_header, buf);
                (&picker_w).render(picker_list, buf);
                (&hint_w).render(hint_row, buf);
            }
        }
    }
}
