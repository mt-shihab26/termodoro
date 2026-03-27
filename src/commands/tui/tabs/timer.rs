use std::io::Result;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Block, Paragraph, Widget};

use crate::commands::tui::tabs::Tab;
use crate::event::Event;
use crate::domains::timer::{LONG_BREAK_INTERVAL, SHOW_MILLIS, TimerState};
use crate::workers::timer_worker;

pub const COLOR: Color = Color::Yellow;

pub struct Timer {
    state: Arc<Mutex<TimerState>>,
}

impl Timer {
    pub fn new(sender: Sender<Event>) -> Self {
        Self {
            state: timer_worker::spawn(sender),
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
        let s = self.state.lock().unwrap();

        let mins = s.millis / 60000;
        let secs = (s.millis / 1000) % 60;
        let ms = (s.millis % 1000) / 10;

        let status = if s.running { "Running" } else { "Paused" };
        let phase = s.phase.label().to_string();
        let time = if SHOW_MILLIS {
            format!("{:02}:{:02}.{:02}", mins, secs, ms)
        } else {
            format!("{:02}:{:02}", mins, secs)
        };
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
        let mut s = self.state.lock().unwrap();
        match key.code {
            KeyCode::Char(' ') => s.running = !s.running,
            KeyCode::Char('r') => {
                s.millis = s.phase.duration();
                s.running = false;
            }
            KeyCode::Char('n') => s.advance(),
            _ => {}
        }
        Ok(())
    }
}
