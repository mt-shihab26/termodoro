use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::{Constraint, Rect}};

use crate::config::Config;
use crate::state::{self, Phase, State};

use super::component::Component;
use super::hints::Hints;
use super::phase_label::PhaseLabel;
use super::progress_bar::ProgressBar;
use super::sessions::Sessions;
use super::status::StatusIndicator;
use super::timer_display::TimerDisplay;
use super::title::Title;

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Running,
    Paused,
}

pub struct TimerWidget {
    state: State,
    pub status: Status,
    config: Config,
}

impl TimerWidget {
    pub fn new(config: Config, saved: Option<State>) -> Self {
        let state = match saved {
            Some(s) => s,
            None => State {
                phase: Phase::Work,
                remaining_secs: config.work_session_duration * 60,
                sessions_completed: 0,
            },
        };
        Self { state, status: Status::Paused, config }
    }

    fn toggle(&mut self) {
        self.status = match self.status {
            Status::Running => Status::Paused,
            Status::Paused => Status::Running,
        };
        self.save();
    }

    fn skip(&mut self) {
        self.advance();
        self.save();
    }

    fn reset(&mut self) {
        self.state.remaining_secs = self.phase_duration();
        self.status = Status::Paused;
        self.save();
    }

    fn advance(&mut self) {
        match self.state.phase {
            Phase::Work => {
                self.state.sessions_completed += 1;
                if self.state.sessions_completed % self.config.long_break_session_interval == 0 {
                    self.state.phase = Phase::LongBreak;
                    self.state.remaining_secs = self.config.long_break_session_duration * 60;
                } else {
                    self.state.phase = Phase::Break;
                    self.state.remaining_secs = self.config.break_session_duration * 60;
                }
            }
            Phase::Break | Phase::LongBreak => {
                self.state.phase = Phase::Work;
                self.state.remaining_secs = self.config.work_session_duration * 60;
            }
        }
        self.status = Status::Paused;
    }

    fn phase_duration(&self) -> u64 {
        match self.state.phase {
            Phase::Work => self.config.work_session_duration * 60,
            Phase::Break => self.config.break_session_duration * 60,
            Phase::LongBreak => self.config.long_break_session_duration * 60,
        }
    }

    fn total_secs(&self) -> u64 {
        self.phase_duration()
    }

    // Effect: persist state to disk after mutations
    fn save(&self) {
        state::save_state(&self.state);
    }
}

impl Component for TimerWidget {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let [_, title, _, phase, _, clock, _, progress, _, sessions, status, _, hints, _] =
            ratatui::layout::Layout::vertical([
                Constraint::Fill(1),   // top spacer
                Constraint::Length(1), // title
                Constraint::Length(2), // gap
                Constraint::Length(1), // phase label
                Constraint::Length(2), // gap
                Constraint::Length(1), // timer
                Constraint::Length(1), // gap
                Constraint::Length(1), // progress bar
                Constraint::Length(2), // gap
                Constraint::Length(1), // sessions
                Constraint::Length(1), // status
                Constraint::Length(2), // gap
                Constraint::Length(1), // hints
                Constraint::Fill(1),   // bottom spacer
            ])
            .areas(area);

        let elapsed = self.total_secs().saturating_sub(self.state.remaining_secs);

        Title.render(frame, title);
        PhaseLabel { phase: &self.state.phase }.render(frame, phase);
        TimerDisplay { remaining_secs: self.state.remaining_secs, status: &self.status }.render(frame, clock);
        ProgressBar { elapsed, total: self.total_secs(), phase: &self.state.phase }.render(frame, progress);
        Sessions { count: self.state.sessions_completed }.render(frame, sessions);
        StatusIndicator { status: &self.status }.render(frame, status);
        Hints.render(frame, hints);
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(' ') => self.toggle(),
            KeyCode::Char('s') => self.skip(),
            KeyCode::Char('r') => self.reset(),
            _ => {}
        }
    }

    fn on_tick(&mut self) {
        if self.status != Status::Running {
            return;
        }
        if self.state.remaining_secs > 0 {
            self.state.remaining_secs -= 1;
        } else {
            self.advance();
            self.save(); // effect: save on phase transition
        }
    }
}
