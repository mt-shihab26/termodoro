use std::io::Result;
use std::sync::{Arc, Mutex};

use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Block, Paragraph, Widget};

use crate::tui::tabs::Tab;
use crate::tui::workers::timer_worker::{self, LONG_BREAK_INTERVAL, TimerWorker};

pub const COLOR: Color = Color::Yellow;

pub struct Timer {
    inner: Arc<Mutex<TimerWorker>>,
}

impl Timer {
    pub fn new(on_tick: impl Fn() + Send + 'static) -> Self {
        Self {
            inner: timer_worker::spawn(on_tick),
        }
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
        let s = self.inner.lock().unwrap();

        let mins = s.seconds / 60;
        let secs = s.seconds % 60;

        let status = if s.running { "Running" } else { "Paused" };
        let phase = s.phase.label().to_string();
        let time = format!("{:02}:{:02}", mins, secs);
        let session = format!("Session {} / {}", s.sessions + 1, LONG_BREAK_INTERVAL);
        let running = s.running;

        drop(s);

        let buf = frame.buffer_mut();
        let hint = "[Space] Toggle   [R] Reset   [N] Skip";

        let block = Block::bordered().fg(COLOR);
        let inner = block.inner(area);
        block.render(area, buf);

        let [phase_row, session_row, time_row, status_row, _, hint_row] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(inner);

        Paragraph::new(phase)
            .alignment(Alignment::Center)
            .bold()
            .fg(COLOR)
            .render(phase_row, buf);
        Paragraph::new(session)
            .alignment(Alignment::Center)
            .fg(Color::DarkGray)
            .render(session_row, buf);
        Paragraph::new(time)
            .alignment(Alignment::Center)
            .bold()
            .fg(COLOR)
            .render(time_row, buf);
        Paragraph::new(status)
            .alignment(Alignment::Center)
            .fg(if running { Color::Green } else { Color::DarkGray })
            .render(status_row, buf);
        Paragraph::new(hint)
            .alignment(Alignment::Center)
            .fg(Color::DarkGray)
            .render(hint_row, buf);
    }

    fn handle(&mut self, key: KeyEvent) -> Result<()> {
        let mut s = self.inner.lock().unwrap();
        match key.code {
            KeyCode::Char(' ') => s.running = !s.running,
            KeyCode::Char('r') => {
                s.seconds = s.phase.duration();
                s.running = false;
            }
            KeyCode::Char('n') => s.advance(),
            _ => {}
        }
        Ok(())
    }
}
