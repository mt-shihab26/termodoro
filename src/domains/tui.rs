use std::io::{Error, ErrorKind, Result};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::time::Duration;

use ratatui::crossterm::event::{KeyCode, KeyModifiers};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph, Widget};
use ratatui::{DefaultTerminal, Frame, init, restore};
use sea_orm::DatabaseConnection;

use crate::tabs::{Tab, timer::Timer, todos::Todos};
use crate::workers::term;
use crate::{config::Config, kinds::event::Event};
use crate::{log_error, widgets::fps::FpsWidget};

pub struct App {
    alive: bool,
    selected: usize,
    tabs: Vec<Box<dyn Tab>>,
    events: Receiver<Event>,
    fps_widget: Option<FpsWidget>,
}

impl App {
    pub fn new(config: Config, db: DatabaseConnection) -> Self {
        let (sender, events) = mpsc::channel::<Event>();

        term::spawn(sender.clone());

        let Config { show_fps, timer, .. } = config;
        let tabs: Vec<Box<dyn Tab>> = vec![Box::new(Todos::new(db)), Box::new(Timer::new(sender, timer))];

        Self {
            alive: true,
            selected: 0,
            tabs,
            events,
            fps_widget: show_fps.then(FpsWidget::new),
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

            let event = if self.tabs[self.selected].should_tick() {
                match self.events.recv_timeout(Duration::from_millis(8)) {
                    Ok(event) => Some(event),
                    Err(RecvTimeoutError::Timeout) => None,
                    Err(RecvTimeoutError::Disconnected) => {
                        log_error!("event channel disconnected");
                        return Err(Error::new(ErrorKind::BrokenPipe, "event channel disconnected"));
                    }
                }
            } else {
                match self.events.recv() {
                    Ok(event) => Some(event),
                    Err(e) => {
                        log_error!("event channel disconnected: {e}");
                        return Err(Error::new(ErrorKind::BrokenPipe, e));
                    }
                }
            };

            match event {
                Some(Event::Key(key)) => match key.code {
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
                Some(Event::Resize(_, _)) => {}
                Some(Event::TimerTick) => {}
                None => {
                    if let Err(e) = self.tabs[self.selected].next_tick() {
                        log_error!("tab tick error: {e}");
                        return Err(e);
                    }
                }
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

        let mut hints = vec![
            Span::from("^q").fg(Color::DarkGray).bold(),
            Span::from(" quit").fg(Color::DarkGray),
        ];

        if self.fps_widget.is_some() {
            hints.push(Span::from("  ").fg(Color::DarkGray));
            hints.push(Span::from("^f").fg(Color::DarkGray).bold());
            hints.push(Span::from(" fps").fg(Color::DarkGray));
        }

        Line::from(hints).render(left, frame.buffer_mut());

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
