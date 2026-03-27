use std::io::Result;
use std::time::Duration;

use ratatui::DefaultTerminal;
use ratatui::Frame;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::Color;
use ratatui::style::{Style, Stylize};
use ratatui::symbols;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Tabs, Widget};

use super::tabs::Tab;
use super::tabs::timer::Timer;
use super::tabs::todos::Todos;

pub struct App {
    alive: bool,
    selected: usize,
    tabs: Vec<Box<dyn Tab>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            alive: true,
            selected: 0,
            tabs: vec![Box::new(Todos), Box::new(Timer::new())],
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
        if event::poll(Duration::from_millis(33))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Char('q') => self.alive = false,
                    KeyCode::Tab => self.selected = (self.selected + 1) % self.tabs.len(),
                    _ => self.tabs[self.selected].handle(key)?,
                },
                _ => {}
            }
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
