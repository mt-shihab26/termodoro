use std::io::{Error, ErrorKind, Result};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Color;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Widget};
use sea_orm::DatabaseConnection;

use crate::kinds::{event::Event, phase::COLOR};
use crate::states::timer_cache::TimerCache;
use crate::widgets::timer::{clock::ClockWidget, status::StatusWidget};
use crate::widgets::timer::{hint::HintWidget, phase::PhaseWidget, session::SessionWidget};
use crate::widgets::timer::{todo::TodoWidget, todo_picker::TodoPickerWidget};
use crate::{config::timer::TimerConfig, states::timer::TimerState};
use crate::{log_error, log_warn, workers::timer::spawn};

use super::Tab;

enum TimerMode {
    Normal,
    SelectingTodo,
}

pub struct TimerTab {
    state: Arc<Mutex<TimerState>>,
    render_count: Arc<AtomicU8>,
    mode: TimerMode,
    selected_todo: Option<(i32, String)>,
    todos: Vec<(i32, String)>,
    todo_cursor: usize,
    cache: Arc<Mutex<TimerCache>>,
}

impl TimerTab {
    pub fn new(
        sender: Sender<Event>,
        timer_config: TimerConfig,
        cache: Arc<Mutex<TimerCache>>,
        db: DatabaseConnection,
    ) -> Self {
        let render_count = Arc::new(AtomicU8::new(1));
        let state = spawn(Arc::clone(&render_count), sender, timer_config, db);

        Self {
            state,
            render_count,
            mode: TimerMode::Normal,
            selected_todo: None,
            todos: Vec::new(),
            todo_cursor: 0,
            cache,
        }
    }

    fn tick_render_count(&self) {
        let current = self.render_count.load(Ordering::Relaxed);
        let next = (current + 1) % u8::MAX;
        self.render_count.store(next, Ordering::Relaxed);
    }

    fn load_todos(&mut self) {
        if let Ok(mut cache) = self.cache.lock() {
            self.todos = cache.todos().to_vec();
        }
        if !self.todos.is_empty() && self.todo_cursor >= self.todos.len() {
            self.todo_cursor = self.todos.len() - 1;
        }
    }

    fn set_selected_todo(&mut self, todo: Option<(i32, String)>) {
        let todo_id = todo.as_ref().map(|(id, _)| *id);
        self.selected_todo = todo;
        if let Ok(mut cache) = self.cache.lock() {
            cache.invalidate_stats();
        }
        if let Ok(mut s) = self.state.lock() {
            s.selected_todo_id = todo_id;
        }
    }

    fn refresh_stats_if_needed(&self, sessions: u32) {
        if let Some((todo_id, _)) = &self.selected_todo {
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
                        s.millis = s.phase.duration(&s.timer_config);
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
                        self.todo_cursor = (self.todo_cursor + 1).min(self.todos.len() - 1);
                    }
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    self.todo_cursor = self.todo_cursor.saturating_sub(1);
                }
                KeyCode::Enter => {
                    if let Some(todo) = self.todos.get(self.todo_cursor).cloned() {
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

        let s = match self.state.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                log_warn!("timer state mutex poisoned in render, recovering");
                poisoned.into_inner()
            }
        };

        let color = s.phase.color();
        let sessions = s.sessions;
        let running = s.running;
        let show_millis = s.timer_config.show_millis();
        let long_break_interval = s.timer_config.long_break_interval();
        let (mins, secs, ms) = s.time_parts();
        let phase_label = s.phase.label().to_string();
        drop(s);

        self.refresh_stats_if_needed(sessions);

        let time = if show_millis {
            format!("{:02}:{:02}.{:02}", mins, secs, ms)
        } else {
            format!("{:02}:{:02}", mins, secs)
        };

        let session_w = SessionWidget {
            session: sessions + 1,
            total: long_break_interval,
        };
        let phase_w = PhaseWidget {
            label: phase_label,
            color,
        };
        let clock_w = ClockWidget { time, color };
        let status_w = StatusWidget { running };
        let hint_w = HintWidget {
            selecting_todo: matches!(self.mode, TimerMode::SelectingTodo),
        };

        let buf = frame.buffer_mut();

        let block = Block::bordered().fg(self.color());
        let inner = block.inner(area);
        block.render(area, buf);
        let area = inner;

        match self.mode {
            TimerMode::Normal => {
                let [
                    session_row,
                    _,
                    phase_row,
                    _,
                    time_row,
                    _,
                    status_row,
                    _,
                    todo_row,
                    hint_row,
                ] = Layout::vertical([
                    Constraint::Length(1),
                    Constraint::Fill(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(8),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Fill(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                ])
                .areas(area);

                let stats = self.cache.lock().ok().and_then(|c| c.stats());
                let todo_w = TodoWidget {
                    selected: self.selected_todo.as_ref().map(|(_, t)| t.as_str()),
                    stats,
                };

                (&session_w).render(session_row, buf);
                (&phase_w).render(phase_row, buf);
                (&clock_w).render(time_row, buf);
                (&status_w).render(status_row, buf);
                (&todo_w).render(todo_row, buf);
                (&hint_w).render(hint_row, buf);
            }

            TimerMode::SelectingTodo => {
                let list_height = (self.todos.len().min(5) as u16).max(1);

                let [
                    session_row,
                    _,
                    phase_row,
                    _,
                    time_row,
                    _,
                    status_row,
                    _,
                    picker_header,
                    picker_list,
                    hint_row,
                ] = Layout::vertical([
                    Constraint::Length(1),
                    Constraint::Fill(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(8),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Fill(1),
                    Constraint::Length(1),
                    Constraint::Length(list_height),
                    Constraint::Length(1),
                ])
                .areas(area);

                let picker_w = TodoPickerWidget {
                    todos: &self.todos,
                    cursor: self.todo_cursor,
                };

                (&session_w).render(session_row, buf);
                (&phase_w).render(phase_row, buf);
                (&clock_w).render(time_row, buf);
                (&status_w).render(status_row, buf);
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
