mod app;
mod tabs;
mod util;

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

use crate::db::Db;
use crate::state;
use crate::timer::Timer;

pub fn run(timer: Timer) -> io::Result<()> {
    let db = Db::open().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "failed to open sqlite db"))?;
    let mut app = app::App::new(timer, db);

    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = event_loop(&mut terminal, &mut app);

    state::save_state(&app.shared.timer.state);

    execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;
    result
}

fn event_loop(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut app::App) -> io::Result<()> {
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|frame| {
            app.render(frame);
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
                    _ => {}
                }
                app.handle_key(key);
            }
        }

        if last_tick.elapsed() >= Duration::from_secs(1) {
            app.on_tick();
            last_tick = Instant::now();
        }
    }

    Ok(())
}
