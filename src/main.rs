use std::io::Result;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{Frame, widgets::Paragraph};

fn main() -> Result<()> {
    let mut terminal = ratatui::init();

    let result = run_app(&mut terminal);

    ratatui::restore();

    result
}

fn run_app(terminal: &mut ratatui::DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(|frame| render(frame))?;
        if handle_events()? {
            break Ok(());
        }
    }
}

fn handle_events() -> Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => return Ok(true),
            _ => {}
        },
        _ => {}
    }
    Ok(false)
}

fn render(frame: &mut Frame) {
    let text = Paragraph::new("Hello World!");
    frame.render_widget(text, frame.area());
}
