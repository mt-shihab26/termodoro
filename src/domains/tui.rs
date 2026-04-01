use std::io::{Error, ErrorKind, Result};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use ratatui::crossterm::event::{KeyCode, KeyModifiers};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::Color;
use ratatui::widgets::Widget;
use ratatui::{DefaultTerminal, Frame, init, restore};
use sea_orm::DatabaseConnection;

use crate::log_error;
use crate::states::timer_cache::TimerCache;
use crate::tabs::{Tab, timer::TimerTab, todos::TodosTab};
use crate::widgets::layout::fps::FpsState;
use crate::widgets::layout::header::{HeaderProps, HeaderWidget};
use crate::widgets::layout::tabs_bar::{TabEntry, TabsBarWidget};
use crate::workers::term;
use crate::{config::Config, kinds::event::Event};

pub struct App {
    alive: bool,
    selected: usize,
    tabs: Vec<Box<dyn Tab>>,
    events: Receiver<Event>,
    fps_show: bool,
    fps_state: FpsState,
}

impl App {
    pub fn new(config: Config, db: DatabaseConnection) -> Self {
        let (sender, events) = mpsc::channel::<Event>();

        term::spawn(sender.clone());

        let timer_cache = Arc::new(Mutex::new(TimerCache::new(db.clone())));

        Self {
            alive: true,
            selected: 0,
            tabs: vec![
                Box::new(TodosTab::new(db.clone(), Arc::clone(&timer_cache))),
                Box::new(TimerTab::new(sender, config.timer, timer_cache, db)),
            ],
            events,
            fps_show: config.show_fps,
            fps_state: FpsState::new(),
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
            if self.fps_show {
                self.fps_state.tick();
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
                        return Err(Error::new(
                            ErrorKind::BrokenPipe,
                            "event channel disconnected",
                        ));
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
                        self.fps_show = !self.fps_show;
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
        let [top, tabs_header, main] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .areas(frame.area());

        HeaderWidget::new(&HeaderProps::new(if self.fps_show {
            Some(&self.fps_state.props())
        } else {
            None
        }))
        .render(top, frame.buffer_mut());

        let tab_entries: Vec<TabEntry> = self
            .tabs
            .iter()
            .enumerate()
            .map(|(i, t)| TabEntry {
                name: t.name(),
                color: if i == self.selected {
                    t.color()
                } else {
                    Color::DarkGray
                },
            })
            .collect();
        (&TabsBarWidget { tabs: &tab_entries }).render(tabs_header, frame.buffer_mut());

        self.tabs[self.selected].render(frame, main);
    }
}
