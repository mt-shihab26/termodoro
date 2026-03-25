use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{self, Write};
use std::time::Duration;

use crate::state;
use crate::timer::{Phase, Timer, TimerState};

pub fn run(mut timer: Timer) -> io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let result = event_loop(&mut stdout, &mut timer);

    state::save(&timer);

    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;
    result
}

fn event_loop(stdout: &mut impl Write, timer: &mut Timer) -> io::Result<()> {
    loop {
        draw(stdout, timer)?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
                match (code, modifiers) {
                    (KeyCode::Char('q'), _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => break,
                    (KeyCode::Char(' '), _) => timer.toggle_pause(),
                    (KeyCode::Char('s'), _) => timer.skip(),
                    (KeyCode::Char('r'), _) => timer.reset(),
                    _ => {}
                }
            }
        }

        if timer.state == TimerState::Running {
            timer.tick();
        }
    }
    Ok(())
}

fn draw(stdout: &mut impl Write, timer: &Timer) -> io::Result<()> {
    let (cols, rows) = terminal::size()?;
    let center_col = cols / 2;
    let center_row = rows / 2;

    queue!(stdout, terminal::Clear(ClearType::All))?;

    // Title
    let title = "termodoro";
    queue!(
        stdout,
        cursor::MoveTo(center_col.saturating_sub(title.len() as u16 / 2), center_row - 5),
        SetForegroundColor(Color::Cyan),
        SetAttribute(Attribute::Bold),
        Print(title),
        ResetColor,
        SetAttribute(Attribute::Reset),
    )?;

    // Phase label
    let phase_label = timer.phase.label();
    let phase_color = match timer.phase {
        Phase::Work => Color::Red,
        Phase::Break => Color::Green,
        Phase::LongBreak => Color::Blue,
    };
    queue!(
        stdout,
        cursor::MoveTo(center_col.saturating_sub(phase_label.len() as u16 / 2), center_row - 3),
        SetForegroundColor(phase_color),
        SetAttribute(Attribute::Bold),
        Print(phase_label),
        ResetColor,
        SetAttribute(Attribute::Reset),
    )?;

    // Timer display
    let mins = timer.remaining_secs / 60;
    let secs = timer.remaining_secs % 60;
    let time_str = format!("{:02}:{:02}", mins, secs);
    let time_color = if timer.state == TimerState::Paused {
        Color::DarkGrey
    } else {
        Color::White
    };
    queue!(
        stdout,
        cursor::MoveTo(center_col.saturating_sub(time_str.len() as u16 / 2), center_row - 1),
        SetForegroundColor(time_color),
        SetAttribute(Attribute::Bold),
        Print(&time_str),
        ResetColor,
        SetAttribute(Attribute::Reset),
    )?;

    // Progress bar
    let bar_width = 30u16;
    let elapsed = timer.total_secs().saturating_sub(timer.remaining_secs);
    let filled = if timer.total_secs() > 0 {
        (elapsed as f64 / timer.total_secs() as f64 * bar_width as f64) as u16
    } else {
        0
    };
    let bar: String = format!(
        "[{}{}]",
        "#".repeat(filled as usize),
        "-".repeat((bar_width - filled) as usize)
    );
    queue!(
        stdout,
        cursor::MoveTo(center_col.saturating_sub(bar_width / 2 + 1), center_row + 1),
        SetForegroundColor(phase_color),
        Print(&bar),
        ResetColor,
    )?;

    // Session count
    let sessions_str = format!("Sessions: {}", timer.sessions_completed);
    queue!(
        stdout,
        cursor::MoveTo(center_col.saturating_sub(sessions_str.len() as u16 / 2), center_row + 3),
        SetForegroundColor(Color::DarkGrey),
        Print(&sessions_str),
        ResetColor,
    )?;

    // Status
    let status = match timer.state {
        TimerState::Running => "● Running",
        TimerState::Paused => "⏸ Paused",
    };
    queue!(
        stdout,
        cursor::MoveTo(center_col.saturating_sub(status.len() as u16 / 2), center_row + 4),
        SetForegroundColor(Color::Yellow),
        Print(status),
        ResetColor,
    )?;

    // Controls hint
    let hint = "[space] pause/resume  [s] skip  [r] reset  [q] quit";
    queue!(
        stdout,
        cursor::MoveTo(center_col.saturating_sub(hint.len() as u16 / 2), center_row + 6),
        SetForegroundColor(Color::DarkGrey),
        Print(hint),
        ResetColor,
    )?;

    stdout.flush()
}
