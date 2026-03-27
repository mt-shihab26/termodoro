mod tabs;

use std::io::Result;
use std::time::Duration;

use ratatui::DefaultTerminal;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::symbols;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Tabs, Widget};

use self::tabs::timer::{self, Timer, TimerState};
use self::tabs::todos::{self, Todos};

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

struct Ui<'a> {
    selected_tab: usize,
    timer:        &'a TimerState,
}

impl<'a> Widget for Ui<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [top, tabs_area, main] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ]).areas(area);

        Line::from_iter([Span::from("Orivo").bold().fg(Color::Green)])
            .centered()
            .render(top, buf);

        let highlight_color = match self.selected_tab {
            0 => todos::COLOR,
            _ => timer::COLOR,
        };

        Tabs::new(vec!["Todos", "Timer"])
            .style(Color::White)
            .highlight_style(Style::default().fg(highlight_color).on_black().bold())
            .select(self.selected_tab)
            .divider(symbols::DOT)
            .padding(" ", " ")
            .render(tabs_area, buf);

        match self.selected_tab {
            0 => Todos.render(main, buf),
            1 => Timer::new(self.timer).render(main, buf),
            _ => unreachable!(),
        }
    }
}
