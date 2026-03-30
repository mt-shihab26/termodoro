use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::calendar::{CalendarEventStore, Monthly};
use ratatui::widgets::{Block, Clear, Paragraph, Widget};
use time::{Date, Duration, Month, OffsetDateTime};

use crate::domains::todos::Repeat;

use super::repeat_picker::{RepeatAction, RepeatPicker};

pub enum CalendarAction {
    Confirm,
    Cancel,
    None,
}

#[derive(Copy, Clone)]
pub struct CalendarPopup {
    pub selected_date: Date,
    pub selected_repeat: Option<Repeat>,
    view_date: Date,
    repeat_picker: Option<RepeatPicker>,
}

impl CalendarPopup {
    pub fn new(date: Option<Date>, repeat: Option<Repeat>) -> Self {
        let d = date.unwrap_or_else(today);

        Self {
            selected_date: d,
            selected_repeat: repeat,
            view_date: d,
            repeat_picker: None,
        }
    }

    pub fn handle(&mut self, key: KeyEvent) -> CalendarAction {
        if let Some(ref mut repeat_picker) = self.repeat_picker {
            match repeat_picker.handle(key) {
                RepeatAction::Confirm(repeat) => {
                    self.repeat_picker = None;
                    self.selected_repeat = repeat;
                    return CalendarAction::Confirm;
                }
                RepeatAction::Cancel => {
                    self.repeat_picker = None;
                }
                RepeatAction::None => {}
            }
            return CalendarAction::None;
        }

        match key.code {
            KeyCode::Char('h') | KeyCode::Left => {
                if let Some(d) = self.selected_date.previous_day() {
                    self.navigate(d);
                }
            }
            KeyCode::Char('l') | KeyCode::Right => {
                if let Some(d) = self.selected_date.next_day() {
                    self.navigate(d);
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if let Some(d) = self.selected_date.checked_sub(Duration::weeks(1)) {
                    self.navigate(d);
                }
            }
            KeyCode::Char('j') | KeyCode::Down => {
                if let Some(d) = self.selected_date.checked_add(Duration::weeks(1)) {
                    self.navigate(d);
                }
            }
            KeyCode::Char('H') => {
                self.navigate(shift_month(self.selected_date, -1));
            }
            KeyCode::Char('L') => {
                self.navigate(shift_month(self.selected_date, 1));
            }
            KeyCode::Char('t') => {
                self.navigate(today());
            }
            KeyCode::Char('y') => {
                if let Some(d) = today().previous_day() {
                    self.navigate(d);
                }
            }
            KeyCode::Char('n') => {
                if let Some(d) = today().next_day() {
                    self.navigate(d);
                }
            }
            KeyCode::Char('r') => {
                self.repeat_picker = Some(RepeatPicker::new(self.selected_repeat));
            }
            KeyCode::Enter => {
                return CalendarAction::Confirm;
            }
            KeyCode::Esc => {
                return CalendarAction::Cancel;
            }
            _ => {}
        }
        CalendarAction::None
    }

    fn navigate(&mut self, date: Date) {
        self.selected_date = date;
        self.view_date = date;
    }
}

impl Widget for CalendarPopup {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup_h = if self.repeat_picker.is_some() {
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
        events.add(self.selected_date, Style::default().bg(Color::Cyan).fg(Color::Black));

        if let Some(picker) = self.repeat_picker {
            picker.render(inner, buf);
            return;
        }

        let [cal_area, nav_hint, action_hint] =
            Layout::vertical([Constraint::Length(8), Constraint::Length(1), Constraint::Length(1)]).areas(inner);

        Monthly::new(self.view_date, events)
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

fn today() -> Date {
    OffsetDateTime::now_local()
        .unwrap_or_else(|_| OffsetDateTime::now_utc())
        .date()
}

fn shift_month(date: Date, delta: i32) -> Date {
    let total = date.month() as i32 - 1 + delta;
    let new_year = date.year() + total.div_euclid(12);
    let new_month_num = (total.rem_euclid(12) + 1) as u8;
    if let Ok(m) = Month::try_from(new_month_num) {
        let new_day = date.day().min(days_in_month(new_year, m));
        if let Ok(d) = Date::from_calendar_date(new_year, m, new_day) {
            return d;
        }
    }
    date
}

fn days_in_month(year: i32, month: Month) -> u8 {
    let (ny, nm) = if month == Month::December {
        (year + 1, 1u8)
    } else {
        (year, month as u8 + 1)
    };
    if let Ok(m) = Month::try_from(nm) {
        if let Ok(first) = Date::from_calendar_date(ny, m, 1) {
            if let Some(last) = first.previous_day() {
                return last.day();
            }
        }
    }
    28
}

fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    Rect {
        x: area.x + area.width.saturating_sub(width) / 2,
        y: area.y + area.height.saturating_sub(height) / 2,
        width: width.min(area.width),
        height: height.min(area.height),
    }
}
