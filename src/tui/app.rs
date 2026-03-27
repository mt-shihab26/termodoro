use std::io::Result;
use std::sync::mpsc;
use std::time::Instant;

use ratatui::Frame;
use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::Color;
use ratatui::style::{Style, Stylize};
use ratatui::symbols;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Tabs, Widget};

use super::event::AppEvent;
use super::tabs::Tab;
use super::tabs::timer::Timer;
use super::tabs::todos::Todos;
use super::workers::term_worker;

pub struct App {
    alive: bool,
    selected: usize,
    tabs: Vec<Box<dyn Tab>>,
    events: mpsc::Receiver<AppEvent>,
    fps: f64,
    frame_count: u32,
    fps_timer: Instant,
}

impl App {
    pub fn new() -> Self {
        let (sender, events) = mpsc::channel::<AppEvent>();

        term_worker::spawn(sender.clone());

        let tabs: Vec<Box<dyn Tab>> = vec![Box::new(Todos), Box::new(Timer::new(sender))];

        Self {
            alive: true,
            selected: 0,
            tabs,
            events,
            fps: 0.0,
            frame_count: 0,
            fps_timer: Instant::now(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut terminal = ratatui::init();

        while self.alive {
            terminal.draw(|frame| self.render_frame(frame))?;

            self.update_fps();

            match self.events.recv() {
                Err(_) => self.alive = false,
                Ok(AppEvent::Term(Event::Key(key))) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Char('q') => self.alive = false,
                    KeyCode::Tab => self.selected = (self.selected + 1) % self.tabs.len(),
                    _ => self.tabs[self.selected].handle(key)?,
                },
                Ok(AppEvent::Tick) => {}
                Ok(_) => {}
            }
        }

        ratatui::restore();
        Ok(())
    }

    fn update_fps(&mut self) {
        self.frame_count += 1;
        let elapsed = self.fps_timer.elapsed().as_secs_f64();
        if elapsed >= 1.0 {
            self.fps = self.frame_count as f64 / elapsed;
            self.frame_count = 0;
            self.fps_timer = Instant::now();
        }
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        let [top, tabs_area, main] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(1), Constraint::Fill(1)]).areas(frame.area());

        Line::from_iter([
            Span::from("Orivo").bold().fg(Color::Green),
            Span::from(format!("  {:.0} fps", self.fps)).fg(Color::DarkGray),
        ])
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
