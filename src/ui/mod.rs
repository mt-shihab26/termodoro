mod app;
mod component;
mod hints;
mod phase_label;
mod progress_bar;
mod sessions;
mod status;
mod timer_display;
mod title;

use std::io;
use std::time::{Duration, Instant};

use ratatui::crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::crossterm::{cursor, execute, terminal};
use ratatui::{Terminal, backend::CrosstermBackend};

use crate::state;
use crate::timer::Timer;

use app::App;
use component::Component;

pub fn run(mut timer: Timer) -> io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = event_loop(&mut terminal, &mut timer);

    state::save_state(&timer.state);

    execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;
    result
}

fn event_loop(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, timer: &mut Timer) -> io::Result<()> {
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|frame| {
            App { timer: &*timer }.render(frame, frame.area());
        })?;

        let elapsed = last_tick.elapsed();
        let timeout = Duration::from_secs(1).saturating_sub(elapsed);

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match (key.code, key.modifiers) {
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => break,
                    _ => {}
                }
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(' ') => timer.toggle(),
                    KeyCode::Char('s') => timer.skip(),
                    KeyCode::Char('r') => timer.reset(),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= Duration::from_secs(1) {
            timer.tick();
            last_tick = Instant::now();
        }
    }

    Ok(())
}
