use std::error::Error;

use crossterm::event::KeyEvent;
use ratatui::prelude::Rect;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, error::SendError, unbounded_channel};
use tracing::{debug, info};

use crate::action::Action;
use crate::cli::Cli;
use crate::components::{Component, fps::FpsCounter, home::Home};
use crate::config::Config;
use crate::tui::{Event, Tui};

type BoxError = Box<dyn Error + Send + Sync>;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[default]
    Home,
}

pub struct App {
    config: Config,
    tick_rate: f64,
    frame_rate: f64,
    components: Vec<Box<dyn Component>>,
    should_quit: bool,
    should_suspend: bool,
    mode: Mode,
    last_tick_key_events: Vec<KeyEvent>,
    action_tx: UnboundedSender<Action>,
    action_rx: UnboundedReceiver<Action>,
}

impl App {
    pub fn new(cli: &Cli) -> Self {
        let (action_tx, action_rx) = unbounded_channel();

        Self {
            tick_rate: cli.tick_rate,
            frame_rate: cli.frame_rate,
            components: vec![Box::new(Home::new()), Box::new(FpsCounter::default())],
            should_quit: false,
            should_suspend: false,
            config: Config::new(),
            mode: Mode::Home,
            last_tick_key_events: Vec::new(),
            action_tx,
            action_rx,
        }
    }

    pub async fn run(&mut self) -> Result<(), BoxError> {
        let mut tui = Tui::new()?
            .mouse(true)
            .tick_rate(self.tick_rate)
            .frame_rate(self.frame_rate);

        tui.enter()?;

        for component in self.components.iter_mut() {
            component.register_action_handler(self.action_tx.clone())?;
        }

        for component in self.components.iter_mut() {
            component.register_config_handler(self.config.clone())?;
        }

        for component in self.components.iter_mut() {
            component.init(tui.size()?)?;
        }

        let action_tx = self.action_tx.clone();

        loop {
            self.handle_events(&mut tui).await?;
            self.handle_actions(&mut tui)?;
            if self.should_suspend {
                tui.suspend()?;
                action_tx.send(Action::Resume)?;
                action_tx.send(Action::ClearScreen)?;
                tui.mouse = true;
                tui.enter()?;
            } else if self.should_quit {
                tui.stop()?;
                break;
            }
        }

        tui.exit()?;

        Ok(())
    }

    async fn handle_events(&mut self, tui: &mut Tui) -> Result<(), BoxError> {
        let Some(event) = tui.next_event().await else {
            return Ok(());
        };

        let action_tx = self.action_tx.clone();

        match event {
            Event::Quit => action_tx.send(Action::Quit)?,
            Event::Tick => action_tx.send(Action::Tick)?,
            Event::Render => action_tx.send(Action::Render)?,
            Event::Resize(x, y) => action_tx.send(Action::Resize(x, y))?,
            Event::Key(key) => self.handle_key_event(key)?,
            _ => {}
        }

        for component in self.components.iter_mut() {
            if let Some(action) = component.handle_events(Some(event.clone()))? {
                action_tx.send(action)?;
            }
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<(), SendError<Action>> {
        let action_tx = self.action_tx.clone();

        let Some(keymap) = self.config.keybindings.0.get(&self.mode) else {
            return Ok(());
        };

        match keymap.get(&vec![key]) {
            Some(action) => {
                info!("Got action: {action:?}");
                action_tx.send(action.clone())?;
            }
            _ => {
                self.last_tick_key_events.push(key);
                if let Some(action) = keymap.get(&self.last_tick_key_events) {
                    info!("Got action: {action:?}");
                    action_tx.send(action.clone())?;
                }
            }
        }

        Ok(())
    }

    fn handle_actions(&mut self, tui: &mut Tui) -> Result<(), BoxError> {
        while let Ok(action) = self.action_rx.try_recv() {
            if action != Action::Tick && action != Action::Render {
                debug!("{action:?}");
            }

            match action {
                Action::Tick => {
                    self.last_tick_key_events.drain(..);
                }
                Action::Quit => self.should_quit = true,
                Action::Suspend => self.should_suspend = true,
                Action::Resume => self.should_suspend = false,
                Action::ClearScreen => tui.terminal.clear()?,
                Action::Resize(w, h) => self.handle_resize(tui, w, h)?,
                Action::Render => self.render(tui)?,
                _ => {}
            }

            for component in self.components.iter_mut() {
                if let Some(action) = component.update(action.clone())? {
                    self.action_tx.send(action)?
                };
            }
        }

        Ok(())
    }

    fn handle_resize(&mut self, tui: &mut Tui, w: u16, h: u16) -> Result<(), std::io::Error> {
        tui.resize(Rect::new(0, 0, w, h))?;
        self.render(tui)?;
        Ok(())
    }

    fn render(&mut self, tui: &mut Tui) -> Result<(), std::io::Error> {
        tui.draw(|frame| {
            for component in self.components.iter_mut() {
                if let Err(err) = component.draw(frame, frame.area()) {
                    let _ = self.action_tx.send(Action::Error(format!("Failed to draw: {:?}", err)));
                }
            }
        })?;
        Ok(())
    }
}
