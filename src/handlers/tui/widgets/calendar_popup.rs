use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::calendar::{CalendarEventStore, Monthly};
use ratatui::widgets::{Block, Clear, Paragraph, Widget};
use time::Date;

use super::repeat_picker::RepeatPicker;

pub enum CalendarAction {
    NavLeft,
    NavRight,
    NavUp,
    NavDown,
    PrevMonth,
    NextMonth,
    Today,
    Yesterday,
    Tomorrow,
    OpenRepeat,
    Confirm,
    Cancel,
    None,
}

pub fn handle(key: KeyEvent) -> CalendarAction {
    match key.code {
        KeyCode::Char('h') | KeyCode::Left => CalendarAction::NavLeft,
        KeyCode::Char('l') | KeyCode::Right => CalendarAction::NavRight,
        KeyCode::Char('j') | KeyCode::Down => CalendarAction::NavDown,
        KeyCode::Char('k') | KeyCode::Up => CalendarAction::NavUp,
        KeyCode::Char('H') => CalendarAction::PrevMonth,
        KeyCode::Char('L') => CalendarAction::NextMonth,
        KeyCode::Char('t') => CalendarAction::Today,
        KeyCode::Char('y') => CalendarAction::Yesterday,
        KeyCode::Char('n') => CalendarAction::Tomorrow,
        KeyCode::Char('r') => CalendarAction::OpenRepeat,
        KeyCode::Enter => CalendarAction::Confirm,
        KeyCode::Esc => CalendarAction::Cancel,
        _ => CalendarAction::None,
    }
}

pub struct CalendarPopup {
    pub selected: Date,
    pub view: Date,
    pub repeat: Option<usize>,
}

impl CalendarPopup {
    pub fn new(selected: Date, view: Date) -> Self {
        Self {
            selected,
            view,
            repeat: None,
        }
    }

    pub fn with_repeat(selected: Date, view: Date, cursor: usize) -> Self {
        Self {
            selected,
            view,
            repeat: Some(cursor),
        }
    }
}

impl Widget for CalendarPopup {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup_h = if self.repeat.is_some() {
            8 + 1 + 1 + 1 + RepeatPicker::height() + 2
        } else {
            8 + 1 + 1 + 2
        };

        let popup = centered_rect(area, 28, popup_h);
        Clear.render(popup, buf);

        let block = Block::bordered()
            .title(" Due Date ")
            .border_style(Style::default().fg(Color::Cyan));
        let inner = block.inner(popup);
        block.render(popup, buf);

        let mut events = CalendarEventStore::today(Style::default().fg(Color::Yellow).bold());
        events.add(self.selected, Style::default().bg(Color::Cyan).fg(Color::Black));

        if let Some(cursor) = self.repeat {
            RepeatPicker::new(cursor).render(inner, buf);
            return;
        }

        let [cal_area, nav_hint, action_hint] =
            Layout::vertical([Constraint::Length(8), Constraint::Length(1), Constraint::Length(1)])
                .areas(inner);

        Monthly::new(self.view, events)
            .show_month_header(Style::default().bold())
            .show_weekdays_header(Style::default().fg(Color::DarkGray))
            .render(cal_area, buf);

        Paragraph::new("[h/l]Day  [j/k]Week  [H/L]Month")
            .centered()
            .fg(Color::DarkGray)
            .render(nav_hint, buf);

        Paragraph::new("[t]Today  [y]Yesterday  [n]Tomorrow  [Enter]Confirm  [r]Repeat  [Esc]Cancel")
            .centered()
            .fg(Color::DarkGray)
            .render(action_hint, buf);
    }
}

fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    Rect {
        x: area.x + area.width.saturating_sub(width) / 2,
        y: area.y + area.height.saturating_sub(height) / 2,
        width: width.min(area.width),
        height: height.min(area.height),
    }
}
