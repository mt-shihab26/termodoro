use std::io::{Error, ErrorKind, Result};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use ratatui::crossterm::event::{KeyCode, KeyModifiers};
use ratatui::prelude::{Color, Constraint, Layout, Line, Span, Stylize, Widget};
use ratatui::style::Style;
use ratatui::widgets::{Block, Paragraph};
use ratatui::{DefaultTerminal, Frame, init, restore};
use sea_orm::DatabaseConnection;

use crate::log_error;
use crate::states::timer_cache::TimerCache;
use crate::tabs::{Tab, timer::TimerTab, todos::TodosTab};
use crate::widgets::layout::fps::{FpsState, FpsWidget};
use crate::workers::term;
use crate::{config::Config, kinds::event::Event};

pub struct App {
    alive: bool,
    selected: usize,
    tabs: Vec<Box<dyn Tab>>,
    events: Receiver<Event>,
    fps_state: Option<FpsState>,
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
            fps_state: config.show_fps.then(FpsState::new),
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
            self.tick_fps();
            self.terminal_draw(terminal)?;
            let event = self.recv_event()?;
            self.handle_event(event)?;
        }

        Ok(())
    }

    fn tick_fps(&mut self) {
        if let Some(fps_state) = &mut self.fps_state {
            fps_state.tick();
        }
    }

    fn terminal_draw(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        if let Err(e) = terminal.draw(|frame| self.render_frame(frame)) {
            log_error!("terminal draw failed: {e}");
            return Err(e);
        }
        Ok(())
    }

    fn recv_event(&self) -> Result<Option<Event>> {
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

        Ok(event)
    }

    fn handle_event(&mut self, event: Option<Event>) -> Result<()> {
        let ctrl = |key: &ratatui::crossterm::event::KeyEvent| {
            key.modifiers.contains(KeyModifiers::CONTROL)
        };

        match event {
            Some(Event::Key(key)) => match key.code {
                KeyCode::Char('c' | 'q') if ctrl(&key) => self.alive = false,
                KeyCode::Char('f') if ctrl(&key) => {
                    self.fps_state = if self.fps_state.is_none() {
                        Some(FpsState::new())
                    } else {
                        None
                    }
                }
                KeyCode::Char('1') if ctrl(&key) => self.selected = 0,
                KeyCode::Char('2') if ctrl(&key) => self.selected = 1,
                KeyCode::Tab => self.selected = (self.selected + 1) % self.tabs.len(),
                _ => self.tabs[self.selected].handle(key).map_err(|e| {
                    log_error!("tab handle error: {e}");
                    e
                })?,
            },
            Some(Event::Resize(_, _)) => {}
            Some(Event::TimerTick) => {}
            None => self.tabs[self.selected].next_tick().map_err(|e| {
                log_error!("tab tick error: {e}");
                e
            })?,
        }

        Ok(())
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let buf = frame.buffer_mut();

        let [top, tabs_header, tab_content] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .areas(area);

        Paragraph::new(Span::from(self.get_app_name()).bold().fg(Color::Green))
            .centered()
            .render(top, buf);

        let [top_left, top_right] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(top);

        Line::from(self.get_hints()).render(top_left, buf);

        if let Some(fps_state) = &self.fps_state {
            FpsWidget::new(fps_state.props()).render(top_right, buf);
        }

        let tab_areas =
            Layout::horizontal(vec![Constraint::Fill(1); self.tabs.len()]).split(tabs_header);

        for index in 0..self.tabs.len() {
            let color = self.get_tab_color(index);

            let block = Block::bordered().border_style(Style::default().fg(color).bold());
            let inner = block.inner(tab_areas[index]);
            block.render(tab_areas[index], buf);

            Paragraph::new(self.tabs[index].name())
                .centered()
                .style(Style::default().fg(color).bold())
                .render(inner, buf);
        }

        self.tabs[self.selected].render(frame, tab_content);
    }

    fn get_app_name(&self) -> &str {
        "Orivo"
    }

    fn get_hints(&self) -> Vec<Span<'static>> {
        let mut hints = vec![
            Span::from("^q").fg(Color::DarkGray).bold(),
            Span::from(" quit").fg(Color::DarkGray),
        ];

        if self.fps_state.is_some() {
            hints.push(Span::from("  ").fg(Color::DarkGray));
            hints.push(Span::from("^f").fg(Color::DarkGray).bold());
            hints.push(Span::from(" fps").fg(Color::DarkGray));
        }

        hints
    }

    fn get_tab_color(&self, index: usize) -> Color {
        if index == self.selected {
            self.tabs[index].color()
        } else {
            Color::DarkGray
        }
    }
}
