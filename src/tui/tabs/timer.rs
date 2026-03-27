use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Block, Paragraph, Widget};

pub const COLOR: Color = Color::Yellow;

const WORK_DURATION: u64 = 25 * 60;
const BREAK_DURATION: u64 = 5 * 60;
const LONG_BREAK_DURATION: u64 = 15 * 60;
const LONG_BREAK_INTERVAL: u32 = 4;

#[derive(Clone, PartialEq)]
pub enum Phase {
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

pub struct TimerState {
    pub phase: Phase,
    pub seconds: u64,
    pub sessions: u32,
    pub running: bool,
}

impl TimerState {
    pub fn new() -> Self {
        Self {
            phase: Phase::Work,
            seconds: WORK_DURATION,
            sessions: 0,
            running: false,
        }
    }

    pub fn tick(&mut self) {
        if !self.running {
            return;
        }
        if self.seconds > 0 {
            self.seconds -= 1;
        } else {
            self.advance();
        }
    }

    pub fn toggle(&mut self) {
        self.running = !self.running;
    }

    pub fn reset(&mut self) {
        self.seconds = self.phase.duration();
        self.running = false;
    }

    pub fn skip(&mut self) {
        self.advance();
    }

    fn advance(&mut self) {
        match self.phase {
            Phase::Work => {
                self.sessions += 1;
                if self.sessions % LONG_BREAK_INTERVAL == 0 {
                    self.phase = Phase::LongBreak;
                } else {
                    self.phase = Phase::Break;
                }
            }
            Phase::Break | Phase::LongBreak => {
                self.phase = Phase::Work;
            }
        }
        self.seconds = self.phase.duration();
        self.running = false;
    }
}

pub struct Timer<'a> {
    pub state: &'a TimerState,
}

impl<'a> Timer<'a> {
    pub fn new(state: &'a TimerState) -> Self {
        Self { state }
    }
}

impl<'a> Widget for Timer<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mins = self.state.seconds / 60;
        let secs = self.state.seconds % 60;

        let status = if self.state.running { "Running" } else { "Paused" };
        let phase = self.state.phase.label();
        let time = format!("{:02}:{:02}", mins, secs);
        let session = format!("Session {} / {}", self.state.sessions + 1, LONG_BREAK_INTERVAL);
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
            .fg(if self.state.running {
                Color::Green
            } else {
                Color::DarkGray
            })
            .render(status_row, buf);

        Paragraph::new(hint)
            .alignment(Alignment::Center)
            .fg(Color::DarkGray)
            .render(hint_row, buf);
    }
}
