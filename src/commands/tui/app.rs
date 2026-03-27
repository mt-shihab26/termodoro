use std::io::{Error, ErrorKind, Result};
use std::sync::mpsc;

use ratatui::crossterm::event::KeyCode;
use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::style::Color;
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph, Widget};
use ratatui::{DefaultTerminal, Frame, init, restore};

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
        let mut terminal = init();

        let result = self.event_loop(&mut terminal);

        restore();

        result
    }

    fn event_loop(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
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
            Layout::vertical([Constraint::Length(1), Constraint::Length(3), Constraint::Fill(1)]).areas(frame.area());

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

        let constraints = vec![Constraint::Fill(1); self.tabs.len()];
        let tab_areas = Layout::horizontal(constraints).split(tabs_area);

        for i in 0..self.tabs.len() {
            let is_selected = i == self.selected;
            let color = if is_selected { self.tabs[i].color() } else { Color::DarkGray };
            let block = Block::bordered().border_style(Style::default().fg(color).bold());
            let inner = block.inner(tab_areas[i]);
            block.render(tab_areas[i], frame.buffer_mut());
            Paragraph::new(self.tabs[i].name())
                .centered()
                .style(Style::default().fg(color).bold())
                .render(inner, frame.buffer_mut());
        }

        self.tabs[self.selected].render(frame, main);
    }
}
