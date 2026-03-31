use std::io::{Error, ErrorKind, Result};
use std::sync::mpsc::{self, Receiver};

use ratatui::crossterm::event::{KeyCode, KeyModifiers};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph, Widget};
use ratatui::{DefaultTerminal, Frame, init, restore};

use crate::config::Config;
use crate::tabs::{Tab, timer::Timer, todos::Todos};
use crate::widgets::fps::FpsWidget;
use crate::{kinds::event::Event, log_error, workers::term};

use super::Cmd;

pub struct Tui {
    config: Config,
}

impl Tui {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl Cmd for Tui {
    fn help(&self) -> &[&str] {
        &["(default)", "tui", "Launch the terminal UI"]
    }

    fn run(&self) -> Result<()> {
        let mut app = App::new(self.config.clone());

        app.run()
    }
}

struct App {
    alive: bool,
    selected: usize,
    tabs: Vec<Box<dyn Tab>>,
    events: Receiver<Event>,
    fps_widget: Option<FpsWidget>,
}

impl App {
    pub fn new(config: Config) -> Self {
        let (sender, events) = mpsc::channel::<Event>();

        term::spawn(sender.clone());

        let tabs: Vec<Box<dyn Tab>> = vec![
            Box::new(Todos::new()),
            Box::new(Timer::new(sender, config.timer.clone())),
        ];

        Self {
            alive: true,
            selected: 0,
            tabs,
            events,
            fps_widget: Some(FpsWidget::new()),
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
            if let Some(fps) = &mut self.fps_widget {
                fps.tick();
            }

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
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.alive = false;
                    }
                    KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.alive = false;
                    }
                    KeyCode::Char('f') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        if self.fps_widget.is_some() {
                            self.fps_widget = None;
                        } else {
                            self.fps_widget = Some(FpsWidget::new());
                        }
                    }
                    KeyCode::Char('t') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.selected = 0;
                    }
                    KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.selected = 1;
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

        let [left, right] = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(top);

        Paragraph::new(Span::from("Orivo").bold().fg(Color::Green))
            .centered()
            .render(top, frame.buffer_mut());

        Line::from(vec![
            Span::from("^q").fg(Color::DarkGray).bold(),
            Span::from(" quit  ").fg(Color::DarkGray),
            Span::from("^f").fg(Color::DarkGray).bold(),
            Span::from(" fps").fg(Color::DarkGray),
        ])
        .render(left, frame.buffer_mut());

        if let Some(fps_widget) = &mut self.fps_widget {
            fps_widget.render(right, frame.buffer_mut());
        }

        let constraints = vec![Constraint::Fill(1); self.tabs.len()];
        let tab_areas = Layout::horizontal(constraints).split(tabs_area);

        for i in 0..self.tabs.len() {
            let is_selected = i == self.selected;
            let color = if is_selected {
                self.tabs[i].color()
            } else {
                Color::DarkGray
            };
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
