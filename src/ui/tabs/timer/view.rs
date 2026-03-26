use ratatui::{
    Frame,
    layout::{Constraint, Rect},
};

use crate::timer::Timer;

use crate::ui::{
    component::Component, hints::Hints, phase_label::PhaseLabel, progress_bar::ProgressBar, sessions::Sessions,
    status::StatusIndicator, timer_display::TimerDisplay, title::Title,
};

pub struct TimerView<'a> {
    pub timer: &'a Timer,
    pub active_todo: Option<&'a str>,
}

impl Component for TimerView<'_> {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let [
            _,
            title,
            _,
            phase,
            focus,
            _,
            time,
            _,
            progress,
            _,
            sessions,
            status,
            _,
            hints,
            _,
        ] = ratatui::layout::Layout::vertical([
            Constraint::Fill(1),   // top spacer
            Constraint::Length(1), // title
            Constraint::Length(2), // gap
            Constraint::Length(1), // phase label
            Constraint::Length(1), // focus
            Constraint::Length(1), // gap
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

        let elapsed = self.timer.total_secs().saturating_sub(self.timer.state.remaining_secs);

        Title.render(frame, title);
        PhaseLabel {
            phase: &self.timer.state.phase,
        }
        .render(frame, phase);
        {
            use ratatui::{
                layout::Alignment,
                style::{Color, Style},
                widgets::Paragraph,
            };
            let text = match self.active_todo {
                Some(t) => format!("On: {t}"),
                None => "On: (no todo selected)".to_string(),
            };
            frame.render_widget(
                Paragraph::new(text)
                    .style(Style::default().fg(Color::DarkGray))
                    .alignment(Alignment::Center),
                focus,
            );
        }
        TimerDisplay {
            remaining_secs: self.timer.state.remaining_secs,
            status: &self.timer.status,
        }
        .render(frame, time);
        ProgressBar {
            elapsed,
            total: self.timer.total_secs(),
            phase: &self.timer.state.phase,
        }
        .render(frame, progress);
        Sessions {
            count: self.timer.state.sessions_completed,
        }
        .render(frame, sessions);
        StatusIndicator {
            status: &self.timer.status,
        }
        .render(frame, status);
        Hints.render(frame, hints);
    }
}
