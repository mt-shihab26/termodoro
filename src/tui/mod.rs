mod tabs;
mod ui;

use std::io::Result;
use std::time::Duration;

use ratatui::DefaultTerminal;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};

use self::tabs::timer::TimerState;
use self::ui::Ui;

pub struct App<'a> {
    alive:        bool,
    terminal:     &'a mut DefaultTerminal,
    selected_tab: usize,
    timer:        TimerState,
}

impl<'a> App<'a> {
    pub fn new(terminal: &'a mut DefaultTerminal) -> Self {
        Self {
            alive: true,
            terminal,
            selected_tab: 0,
            timer: TimerState::new(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        while self.alive {
            self.render_pixels()?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_secs(1))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Char('q') => self.alive = false,
                        KeyCode::Char('1') => self.selected_tab = 0,
                        KeyCode::Char('2') => self.selected_tab = 1,
                        _ => {
                            if self.selected_tab == 1 {
                                self.timer.handle_event(key);
                            }
                        }
                    }
                }
                _ => {}
            }
        } else {
            self.timer.tick();
        }
        Ok(())
    }

    fn render_pixels(&mut self) -> Result<()> {
        let selected_tab = self.selected_tab;
        let timer = &self.timer;
        self.terminal.draw(|frame| {
            frame.render_widget(Ui { selected_tab, timer }, frame.area());
        })?;
        Ok(())
    }
}
