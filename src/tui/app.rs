use std::io::Result;
use std::time::Duration;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style, Stylize};
use ratatui::symbols;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Tabs, Widget};
use ratatui::{DefaultTerminal, Frame};

use super::tabs::timer::{self, Timer, TimerState};
use super::tabs::todos::{self, Todos};

pub struct App {
    alive: bool,
    selected: usize,
    timer: TimerState,
}

impl App {
    pub fn new() -> Self {
        Self {
            alive: true,
            selected: 0,
            timer: TimerState::new(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut terminal = ratatui::init();

        while self.alive {
            self.render_pixels(&mut terminal)?;
            self.handle_events()?;
        }

        ratatui::restore();

        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_secs(1))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Char('q') => self.alive = false,
                    KeyCode::Char('1') => self.selected = 0,
                    KeyCode::Char('2') => self.selected = 1,
                    _ => {
                        if self.selected == 1 {
                            self.timer.handle_event(key);
                        }
                    }
                },
                _ => {}
            }
        } else {
            self.timer.tick();
        }
        Ok(())
    }

    fn render_pixels(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        terminal.draw(|frame| self.render_frame(frame))?;
        Ok(())
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        let [top, tabs_area, main] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(1), Constraint::Fill(1)]).areas(frame.area());

        Line::from_iter([Span::from("Orivo").bold().fg(Color::Green)])
            .centered()
            .render(top, frame.buffer_mut());

        let highlight_color = match self.selected {
            0 => todos::COLOR,
            _ => timer::COLOR,
        };

        Tabs::new(vec!["Todos", "Timer"])
            .style(Color::White)
            .highlight_style(Style::default().fg(highlight_color).on_black().bold())
            .select(self.selected)
            .divider(symbols::DOT)
            .padding(" ", " ")
            .render(tabs_area, frame.buffer_mut());

        match self.selected {
            0 => Todos.render(main, frame.buffer_mut()),
            1 => Timer::new(&self.timer).render(main, frame.buffer_mut()),
            _ => unreachable!(),
        }
    }
}
