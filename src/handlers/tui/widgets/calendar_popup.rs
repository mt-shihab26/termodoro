use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::calendar::{CalendarEventStore, Monthly};
use ratatui::widgets::{Block, Clear, Paragraph, Widget};
use time::{Date, Duration};

use crate::domains::todos::Repeat;
use crate::utils::date::{shift_month, today};

use super::repeat_picker::{RepeatAction, RepeatPicker};

pub enum CalendarAction {
    Confirm { date: Date, repeat: Option<Repeat> },
    Cancel,
    None,
}

#[derive(Copy, Clone)]
pub struct CalendarPopup {
    selected_date: Date,
    selected_repeat: Option<Repeat>,
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
                    return CalendarAction::Confirm {
                        date: self.selected_date,
                        repeat: self.selected_repeat,
                    };
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
                return CalendarAction::Confirm {
                    date: self.selected_date,
                    repeat: self.selected_repeat,
                };
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
        let popup = centered_rect(area, 24, 4 + 10 + 6);

        Clear.render(popup, buf);

        let block = Block::bordered()
            .title(" Due Date ")
            .border_style(Style::default().fg(Color::Cyan));

        let inner = block.inner(popup);
        block.render(popup, buf);

        let mut events = CalendarEventStore::today(Style::default().fg(Color::Yellow).bold());
        events.add(self.selected_date, Style::default().bg(Color::Cyan).fg(Color::Black));

        if let Some(repeat_picker) = self.repeat_picker {
            repeat_picker.render(inner, buf);
            return;
        }

        let [action_hint, cal_area, action_hint2, nav_hint] = Layout::vertical([
            Constraint::Length(4),
            Constraint::Length(10),
            Constraint::Length(1),
            Constraint::Length(6),
        ])
        .areas(inner);

        Paragraph::new("[t]Today\n[y]Yesterday\n[n]Tomorrow").render(action_hint, buf);

        Monthly::new(self.view_date, events)
            .show_month_header(Style::default().bold())
            .show_weekdays_header(Style::default().fg(Color::DarkGray))
            .render(cal_area, buf);

        Paragraph::new("[r]Repeat\n").render(action_hint2, buf);

        Paragraph::new(
            "[h/l]Day\n\
            [j/k]Week\n\
            [H/L]Month\n\
            [Enter]Confirm\n\
            [Esc]Cancel",
        )
        .fg(Color::DarkGray)
        .render(nav_hint, buf);
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
