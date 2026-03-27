use std::io::{Error, ErrorKind, Result};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, Paragraph, Widget};
use tui_big_text::{BigText, PixelSize};

use crate::commands::tui::tabs::Tab;
use crate::domains::timer::{LONG_BREAK_INTERVAL, SHOW_MILLIS, TimerState};
use crate::event::Event;
use crate::workers::timer_worker;
use crate::{log_error, log_warn};

pub const COLOR: Color = Color::Yellow;

pub struct Timer {
    state: Arc<Mutex<TimerState>>,
    render_count: Arc<AtomicU8>,
}

impl Timer {
    pub fn new(sender: Sender<Event>) -> Self {
        let render_count = Arc::new(AtomicU8::new(1));

        let state = timer_worker::spawn(Arc::clone(&render_count), sender);

        Self { state, render_count }
    }

    fn tick_render_count(&self) {
        let current = self.render_count.load(Ordering::Relaxed);
        let next = (current + 1) % u8::MAX;
        self.render_count.store(next, Ordering::Relaxed);
    }
}

impl Tab for Timer {
    fn name(&self) -> &str {
        "Timer"
    }

    fn color(&self) -> Color {
        COLOR
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

        let buf = frame.buffer_mut();

        let block = Block::bordered().fg(COLOR);
        let inner = block.inner(area);
        block.render(area, buf);

        let [session_row, _, phase_row, _, time_row, _, status_row, _, hint_row] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(8),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(inner);

        Paragraph::new(format!("Session {} / {}", s.sessions + 1, LONG_BREAK_INTERVAL))
            .centered()
            .fg(Color::DarkGray)
            .render(session_row, buf);

        Paragraph::new(s.phase.label().to_string())
            .centered()
            .bold()
            .fg(COLOR)
            .render(phase_row, buf);

        let (mins, secs, ms) = s.time_parts();

        let time = if SHOW_MILLIS {
            format!("{:02}:{:02}.{:02}", mins, secs, ms)
        } else {
            format!("{:02}:{:02}", mins, secs)
        };

        BigText::builder()
            .pixel_size(PixelSize::Full)
            .style(Style::new().fg(COLOR).bold())
            .lines(vec![time.as_str().into()])
            .centered()
            .build()
            .render(time_row, buf);

        let status = if s.running { "Running" } else { "Paused" };

        Paragraph::new(status)
            .centered()
            .fg(if s.running { Color::Green } else { Color::DarkGray })
            .render(status_row, buf);

        Paragraph::new("[Space] Toggle   [R] Reset   [N] Skip")
            .centered()
            .fg(Color::DarkGray)
            .render(hint_row, buf);

        drop(s);
    }

    fn handle(&mut self, key: KeyEvent) -> Result<()> {
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
                s.millis = s.phase.duration();
                s.running = false;
            }
            KeyCode::Char('n') => {
                s.advance();
            }
            _ => {}
        }
        Ok(())
    }
}
