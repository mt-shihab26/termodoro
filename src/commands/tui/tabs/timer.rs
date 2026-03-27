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
            Constraint::Length(12),
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

        let rows = big_digits(&time);

        Paragraph::new(Text::from(rows.map(Line::from).to_vec()))
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

fn big_digits(text: &str) -> [String; 12] {
    #[rustfmt::skip]
    const DIGITS: [[&str; 12]; 10] = [
        [" ▄█████▄ ", "▄██   ██▄", "██     ██", "██     ██", "██     ██", "██     ██", "██     ██", "██     ██", "██     ██", "██     ██", "▀██   ██▀", " ▀█████▀ "], // 0
        ["   ▄██   ", "  ████   ", "   ██    ", "   ██    ", "   ██    ", "   ██    ", "   ██    ", "   ██    ", "   ██    ", "   ██    ", "   ██    ", "█████████"], // 1
        [" ▄█████▄ ", "▄██   ██▄", "██     ██", "      ███", "     ███ ", "    ███  ", "   ███   ", "  ███    ", " ███     ", "███      ", "██       ", "█████████"], // 2
        [" ▄█████▄ ", "▄██   ██▄", "██     ██", "       ██", "      ███", "   █████ ", "   █████ ", "      ███", "       ██", "██     ██", "▀██   ██▀", " ▀█████▀ "], // 3
        ["██      █", "██      █", "██      █", "██      █", "██     ██", "███   ███", "█████████", "       ██", "       ██", "       ██", "       ██", "       ██"], // 4
        ["█████████", "██       ", "██       ", "██       ", "████████ ", "      ███", "       ██", "       ██", "       ██", "██     ██", "▀██   ██▀", " ▀█████▀ "], // 5
        [" ▄█████▄ ", "▄██   ██▄", "██     ██", "██       ", "████████ ", "██     ██", "██     ██", "██     ██", "██     ██", "██     ██", "▀██   ██▀", " ▀█████▀ "], // 6
        ["█████████", "       ██", "      ██ ", "     ██  ", "    ██   ", "   ██    ", "   ██    ", "   ██    ", "   ██    ", "   ██    ", "   ██    ", "   ██    "], // 7
        [" ▄█████▄ ", "▄██   ██▄", "██     ██", "██     ██", "▀██   ██▀", " ▄█████▄ ", "▄██   ██▄", "██     ██", "██     ██", "██     ██", "▀██   ██▀", " ▀█████▀ "], // 8
        [" ▄█████▄ ", "▄██   ██▄", "██     ██", "██     ██", "██     ██", " ████████", "       ██", "       ██", "       ██", "██     ██", "▀██   ██▀", " ▀█████▀ "], // 9
    ];

    let mut rows: [String; 12] = Default::default();
    let mut first = true;

    for ch in text.chars() {
        let cols: [&str; 12] = match ch {
            '0'..='9' => DIGITS[(ch as u8 - b'0') as usize],
            ':' => ["   ", "   ", " █ ", " █ ", "   ", "   ", "   ", "   ", " █ ", " █ ", "   ", "   "],
            '.' => ["   ", "   ", "   ", "   ", "   ", "   ", "   ", "   ", "   ", "   ", " █ ", " █ "],
            _ => ["         "; 12],
        };
        for (row, col) in rows.iter_mut().zip(cols) {
            if !first {
                row.push(' ');
            }
            row.push_str(col);
        }
        first = false;
    }

    rows
}
