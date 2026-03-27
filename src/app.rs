use std::io::Result;

use ratatui::DefaultTerminal;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::{Alignment, Constraint, Layout, Offset, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph, Tabs};
use ratatui::{Frame, symbols};

pub struct App<'a> {
    alive: bool,
    terminal: &'a mut DefaultTerminal,
    selected_tab: usize,
}

impl<'a> App<'a> {
    pub fn new(terminal: &'a mut DefaultTerminal) -> Self {
        Self {
            alive: true,
            terminal,
            selected_tab: 0,
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
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Char('q') => self.alive = false,
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }

    fn render_pixels(&mut self) -> Result<()> {
        self.terminal.draw(|frame| render_frame(frame, self.selected_tab))?;

        Ok(())
    }
}

fn render_frame(frame: &mut Frame, selected_tab: usize) {
    let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);

    let [top, main] = frame.area().layout(&layout);

    let title = Line::from_iter([
        Span::from("Tabs Widget").bold(),
        Span::from(" (Press 'q' to quit, arrow keys to navigate tabs)"),
    ]);

    frame.render_widget(title.centered(), top);

    render_content(frame, main, selected_tab);
    render_tabs(frame, main + Offset::new(1, 0), selected_tab);
}

fn render_tabs(frame: &mut Frame, area: Rect, selected_tab: usize) {
    let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3"])
        .style(Color::White)
        .highlight_style(Style::default().magenta().on_black().bold())
        .select(selected_tab)
        .divider(symbols::DOT)
        .padding(" ", " ");

    frame.render_widget(tabs, area);
}

fn render_content(frame: &mut Frame, area: Rect, selected_tab: usize) {
    let text = match selected_tab {
        0 => "Great terminal interfaces start with a single widget.".into(),
        1 => "In the terminal, we don't just render widgets; we create dreams.".into(),
        2 => "Render boldly, style with purpose.".bold(),
        _ => unreachable!(),
    };

    let block = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(Block::bordered());

    frame.render_widget(block, area);
}
