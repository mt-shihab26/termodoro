use std::cell::RefCell;
use std::io::{Error, ErrorKind, Result};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, List, ListItem, Paragraph, Widget};
use sea_orm::DatabaseConnection;
use tui_big_text::{BigText, PixelSize};

use crate::kinds::{event::Event, page::Page, phase::COLOR};
use crate::models::{timer, todo::Todo};
use crate::{config::timer::TimerConfig, states::timer::TimerState};
use crate::{log_error, log_warn, workers::timer::spawn};

use super::Tab;

enum TimerMode {
    Normal,
    SelectingTodo,
}

pub struct Timer {
    db: DatabaseConnection,
    state: Arc<Mutex<TimerState>>,
    render_count: Arc<AtomicU8>,
    mode: TimerMode,
    selected_todo: Option<(i32, String)>,
    todos: Vec<(i32, String)>,
    todo_cursor: usize,
    // (sessions_count_at_cache_time, cached_stats) — invalidated when sessions changes
    cached_stats: RefCell<(u32, Option<(u32, u32)>)>,
}

impl Timer {
    pub fn new(sender: Sender<Event>, timer_config: TimerConfig, db: DatabaseConnection) -> Self {
        let render_count = Arc::new(AtomicU8::new(1));
        let state = spawn(Arc::clone(&render_count), sender, timer_config, db.clone());

        Self {
            db,
            state,
            render_count,
            mode: TimerMode::Normal,
            selected_todo: None,
            todos: Vec::new(),
            todo_cursor: 0,
            cached_stats: RefCell::new((u32::MAX, None)),
        }
    }

    fn tick_render_count(&self) {
        let current = self.render_count.load(Ordering::Relaxed);
        let next = (current + 1) % u8::MAX;
        self.render_count.store(next, Ordering::Relaxed);
    }

    fn load_todos(&mut self) {
        let todos = Todo::list(&self.db, Page::Today, 0, 100);
        self.todos = todos
            .into_iter()
            .filter_map(|t| t.id.map(|id| (id, t.text.clone())))
            .collect();
        if !self.todos.is_empty() && self.todo_cursor >= self.todos.len() {
            self.todo_cursor = self.todos.len() - 1;
        }
    }

    fn set_selected_todo(&mut self, todo: Option<(i32, String)>) {
        let todo_id = todo.as_ref().map(|(id, _)| *id);
        self.selected_todo = todo;
        *self.cached_stats.borrow_mut() = (u32::MAX, None);
        if let Ok(mut s) = self.state.lock() {
            s.selected_todo_id = todo_id;
        }
    }

    fn refresh_stats_if_needed(&self, sessions: u32) {
        if let Some((todo_id, _)) = &self.selected_todo {
            let mut cache = self.cached_stats.borrow_mut();
            if cache.1.is_none() || cache.0 != sessions {
                cache.1 = Some(timer::stats_for_todo(&self.db, *todo_id));
                cache.0 = sessions;
            }
        }
    }
}

impl Tab for Timer {
    fn name(&self) -> &str {
        "Timer [^p]"
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
                        s.advance();
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

                Paragraph::new(format!(
                    "Session {} / {}",
                    sessions + 1,
                    long_break_interval
                ))
                .centered()
                .fg(Color::DarkGray)
                .render(session_row, buf);

                Paragraph::new(phase_label)
                    .centered()
                    .bold()
                    .fg(color)
                    .render(phase_row, buf);

                BigText::builder()
                    .pixel_size(PixelSize::Full)
                    .style(Style::new().fg(color).bold())
                    .lines(vec![time.as_str().into()])
                    .centered()
                    .build()
                    .render(time_row, buf);

                Paragraph::new(if running { "Running" } else { "Paused" })
                    .centered()
                    .fg(if running {
                        Color::Green
                    } else {
                        Color::DarkGray
                    })
                    .render(status_row, buf);

                let todo_line = match &self.selected_todo {
                    Some((_, text)) => {
                        let cache = self.cached_stats.borrow();
                        match cache.1 {
                            Some((count, total_secs)) => {
                                format!(
                                    "{}  ·  {} sessions  ·  {} min",
                                    text,
                                    count,
                                    total_secs / 60
                                )
                            }
                            None => text.clone(),
                        }
                    }
                    None => "No todo selected  [t] pick".to_string(),
                };
                Paragraph::new(todo_line)
                    .centered()
                    .fg(Color::DarkGray)
                    .render(todo_row, buf);

                Paragraph::new("[Space] Toggle   [r] Reset   [n] Skip   [t] Todo   [T] Clear todo")
                    .centered()
                    .fg(Color::DarkGray)
                    .render(hint_row, buf);
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
                    todo_header,
                    todo_list_area,
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

                Paragraph::new(format!(
                    "Session {} / {}",
                    sessions + 1,
                    long_break_interval
                ))
                .centered()
                .fg(Color::DarkGray)
                .render(session_row, buf);

                Paragraph::new(phase_label)
                    .centered()
                    .bold()
                    .fg(color)
                    .render(phase_row, buf);

                BigText::builder()
                    .pixel_size(PixelSize::Full)
                    .style(Style::new().fg(color).bold())
                    .lines(vec![time.as_str().into()])
                    .centered()
                    .build()
                    .render(time_row, buf);

                Paragraph::new(if running { "Running" } else { "Paused" })
                    .centered()
                    .fg(if running {
                        Color::Green
                    } else {
                        Color::DarkGray
                    })
                    .render(status_row, buf);

                Paragraph::new("Select a todo")
                    .centered()
                    .bold()
                    .fg(Color::Yellow)
                    .render(todo_header, buf);

                if self.todos.is_empty() {
                    Paragraph::new("No todos for today")
                        .centered()
                        .fg(Color::DarkGray)
                        .render(todo_list_area, buf);
                } else {
                    let start = self
                        .todo_cursor
                        .saturating_sub(2)
                        .min(self.todos.len().saturating_sub(5));

                    let items: Vec<ListItem> = self
                        .todos
                        .iter()
                        .enumerate()
                        .skip(start)
                        .take(5)
                        .map(|(i, (_, text))| {
                            let is_cursor = i == self.todo_cursor;
                            let prefix = if is_cursor { "> " } else { "  " };
                            let style = if is_cursor {
                                Style::new().fg(Color::Yellow).bold()
                            } else {
                                Style::new().fg(Color::DarkGray)
                            };
                            ListItem::new(format!("{prefix}{text}")).style(style)
                        })
                        .collect();

                    List::new(items).render(todo_list_area, buf);
                }

                Paragraph::new("[j/k] Navigate   [Enter] Select   [Esc] Cancel")
                    .centered()
                    .fg(Color::DarkGray)
                    .render(hint_row, buf);
            }
        }
    }
}
