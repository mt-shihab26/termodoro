use std::io::Result;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Stylize};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Paragraph, Widget};

use crate::commands::tui::tabs::Tab;
use crate::domains::timer::{LONG_BREAK_INTERVAL, SHOW_MILLIS, TimerState};
use crate::event::Event;
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

        let buf = frame.buffer_mut();

        let block = Block::bordered().fg(COLOR);

        let inner = block.inner(area);

        block.render(area, buf);

        let [session_row, _, phase_row, time_row, status_row, _, hint_row] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(inner);

        Paragraph::new(format!("Session {} / {}", s.sessions + 1, LONG_BREAK_INTERVAL))
            .alignment(Alignment::Center)
            .fg(Color::DarkGray)
            .render(session_row, buf);

        Paragraph::new(s.phase.label().to_string())
            .alignment(Alignment::Center)
            .bold()
            .fg(COLOR)
            .render(phase_row, buf);

        let (mins, secs, ms) = s.time_parts();

        let time = if SHOW_MILLIS {
            format!("{:02}:{:02}.{:02}", mins, secs, ms)
        } else {
            format!("{:02}:{:02}", mins, secs)
        };

        let [r0, r1, r2] = big_digits(&time);

        Paragraph::new(Text::from(vec![Line::from(r0), Line::from(r1), Line::from(r2)]))
            .alignment(Alignment::Center)
            .bold()
            .fg(COLOR)
            .render(time_row, buf);

        let status = if s.running { "Running" } else { "Paused" };

        Paragraph::new(status)
            .alignment(Alignment::Center)
            .fg(if s.running { Color::Green } else { Color::DarkGray })
            .render(status_row, buf);

        Paragraph::new("[Space] Toggle   [R] Reset   [N] Skip")
            .alignment(Alignment::Center)
            .fg(Color::DarkGray)
            .render(hint_row, buf);

        drop(s);
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

fn big_digits(text: &str) -> [String; 3] {
    const DIGITS: [(&str, &str, &str); 10] = [
        (" _ ", "| |", "|_|"),
        ("   ", " | ", " | "),
        (" _ ", " _|", "|_ "),
        (" _ ", " _|", " _|"),
        ("   ", "|_|", "  |"),
        (" _ ", "|_ ", " _|"),
        (" _ ", "|_ ", "|_|"),
        (" _ ", "  |", "  |"),
        (" _ ", "|_|", "|_|"),
        (" _ ", "|_|", " _|"),
    ];

    let mut rows = [String::new(), String::new(), String::new()];

    for ch in text.chars() {
        let (r0, r1, r2) = match ch {
            '0'..='9' => DIGITS[(ch as u8 - b'0') as usize],
            ':' => (" ", ":", ":"),
            '.' => (" ", " ", "."),
            _ => ("   ", "   ", "   "),
        };
        rows[0].push_str(r0);
        rows[1].push_str(r1);
        rows[2].push_str(r2);
    }

    rows
}
