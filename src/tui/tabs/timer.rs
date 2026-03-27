use std::io::Result;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Block, Paragraph, Widget};

use ratatui::Frame;

use crate::tui::tabs::Tab;

pub const COLOR: Color = Color::Yellow;

const WORK_DURATION: u64 = 25 * 60;
const BREAK_DURATION: u64 = 5 * 60;
const LONG_BREAK_DURATION: u64 = 15 * 60;
const LONG_BREAK_INTERVAL: u32 = 4;

#[derive(Clone, PartialEq)]
enum Phase {
    Work,
    Break,
    LongBreak,
}

impl Phase {
    fn label(&self) -> &str {
        match self {
            Phase::Work => "Work Session",
            Phase::Break => "Short Break",
            Phase::LongBreak => "Long Break",
        }
    }

    fn duration(&self) -> u64 {
        match self {
            Phase::Work => WORK_DURATION,
            Phase::Break => BREAK_DURATION,
            Phase::LongBreak => LONG_BREAK_DURATION,
        }
    }
}

struct TimerInner {
    phase: Phase,
    seconds: u64,
    sessions: u32,
    running: bool,
}

impl TimerInner {
    fn tick(&mut self) {
        if !self.running {
            return;
        }
        if self.seconds > 0 {
            self.seconds -= 1;
        } else {
            self.advance();
        }
    }

    fn advance(&mut self) {
        match self.phase {
            Phase::Work => {
                self.sessions += 1;
                self.phase = if self.sessions % LONG_BREAK_INTERVAL == 0 {
                    Phase::LongBreak
                } else {
                    Phase::Break
                };
            }
            Phase::Break | Phase::LongBreak => {
                self.phase = Phase::Work;
            }
        }
        self.seconds = self.phase.duration();
        self.running = false;
    }
}

pub struct Timer {
    inner: Arc<Mutex<TimerInner>>,
}

impl Timer {
    pub fn new() -> Self {
        let inner = Arc::new(Mutex::new(TimerInner {
            phase: Phase::Work,
            seconds: WORK_DURATION,
            sessions: 0,
            running: false,
        }));

        let thread_inner = Arc::clone(&inner);

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(1));
                thread_inner.lock().unwrap().tick();
            }
        });

        Self { inner }
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

        let [_, center, _] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(10), Constraint::Fill(1)]).areas(area);

        let block = Block::bordered().fg(COLOR);
        let inner = block.inner(center);
        block.render(center, buf);

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
