use std::io::{Error, ErrorKind, Result};
use std::sync::mpsc;

use ratatui::Frame;
use ratatui::crossterm::event::KeyCode;
use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::style::Color;
use ratatui::style::{Style, Stylize};
use ratatui::symbols;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Tabs, Widget};

use crate::event::Event;
use crate::log_error;
use crate::workers::term_worker;

use super::fps::Fps;
use super::tabs::Tab;
use super::tabs::timer::Timer;
use super::tabs::todos::Todos;

pub struct App {
    alive: bool,
    selected: usize,
    tabs: Vec<Box<dyn Tab>>,
    events: mpsc::Receiver<Event>,
    fps: Fps,
}

impl App {
    pub fn new() -> Self {
        let (sender, events) = mpsc::channel::<Event>();

        term_worker::spawn(sender.clone());

        let tabs: Vec<Box<dyn Tab>> = vec![Box::new(Todos), Box::new(Timer::new(sender))];

        Self {
            alive: true,
            selected: 1,
            tabs,
            events,
            fps: Fps::new(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut terminal = ratatui::init();

        let result = self.event_loop(&mut terminal);

        ratatui::restore();

        result
    }

    fn event_loop(&mut self, terminal: &mut ratatui::DefaultTerminal) -> Result<()> {
        while self.alive {
            self.fps.tick();

            if let Err(e) = terminal.draw(|frame| self.render_frame(frame)) {
                log_error!("terminal draw failed: {e}");
                return Err(e);
            }

            match self.events.recv() {
                Err(e) => {
                    log_error!("event channel disconnected: {e}");
                    return Err(Error::new(ErrorKind::BrokenPipe, e));
                }
                Ok(Event::Key(key)) => match key.code {
                    KeyCode::Char('q') => {
                        self.alive = false;
                    }
                    KeyCode::Char('f') => {
                        self.fps.visible = !self.fps.visible;
                    }
                    KeyCode::Tab => {
                        self.selected = (self.selected + 1) % self.tabs.len();
                    }
                    _ => {
                        if let Err(e) = self.tabs[self.selected].handle(key) {
                            log_error!("tab handle error: {e}");
                            return Err(e);
                        }
                    }
                },
                Ok(Event::Tick) => {}
            }
        }

        Ok(())
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        let [top, tabs_area, main] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(1), Constraint::Fill(1)]).areas(frame.area());

        let [_, right] = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(top);

        Paragraph::new(Span::from("Orivo").bold().fg(Color::Green))
            .centered()
            .render(top, frame.buffer_mut());

        if self.fps.visible {
            Line::from(
                Span::from(format!(
                    "{:.0} fps  {} frames",
                    self.fps.per_second, self.fps.per_lifetime
                ))
                .fg(Color::DarkGray),
            )
            .alignment(Alignment::Right)
            .render(right, frame.buffer_mut());
        }

        let highlight_color = self.tabs[self.selected].color();
        let names: Vec<&str> = self.tabs.iter().map(|t| t.name()).collect();

        Tabs::new(names)
            .style(Color::White)
            .highlight_style(Style::default().fg(highlight_color).on_black().bold())
            .select(self.selected)
            .divider(symbols::DOT)
            .padding(" ", " ")
            .render(tabs_area, frame.buffer_mut());

        self.tabs[self.selected].render(frame, main);
    }
}
