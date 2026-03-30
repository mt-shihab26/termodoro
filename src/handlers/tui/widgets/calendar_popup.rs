use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::calendar::{CalendarEventStore, Monthly};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Widget};
use time::{Date, Duration};

use crate::domains::todos::repeat::Repeat;
use crate::utils::date::{shift_month, today};

use super::repeat_picker::{RepeatAction, RepeatPicker};

pub enum CalendarAction {
    Confirm { date: Option<Date>, repeat: Option<Repeat> },
    Cancel,
    None,
}

pub struct CalendarPopup {
    date: Date,
    repeat: Option<Repeat>,
    repeat_picker: Option<RepeatPicker>,
}

impl CalendarPopup {
    pub fn new(date: Option<Date>, repeat: Option<Repeat>) -> Self {
        let d = date.unwrap_or_else(today);

        Self {
            repeat,
            date: d,
            repeat_picker: None,
        }
    }

    pub fn handle(&mut self, key: KeyEvent) -> CalendarAction {
        if let Some(ref mut repeat_picker) = self.repeat_picker {
            match repeat_picker.handle(key) {
                RepeatAction::Confirm(repeat) => {
                    self.repeat_picker = None;
                    self.repeat = repeat;
                    return CalendarAction::Confirm {
                        date: Some(self.date),
                        repeat: repeat,
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
            KeyCode::Char('x') => self.navigate(None),
            KeyCode::Char('t') => self.navigate(Some(today())),
            KeyCode::Char('y') => self.navigate(today().previous_day()),
            KeyCode::Char('n') => self.navigate(today().next_day()),
            KeyCode::Char('h') | KeyCode::Left => self.navigate(self.date.previous_day()),
            KeyCode::Char('l') | KeyCode::Right => self.navigate(self.date.next_day()),
            KeyCode::Char('k') | KeyCode::Up => self.navigate(self.date.checked_sub(Duration::weeks(1))),
            KeyCode::Char('j') | KeyCode::Down => self.navigate(self.date.checked_add(Duration::weeks(1))),
            KeyCode::Char('H') => self.navigate(Some(shift_month(self.date, -1))),
            KeyCode::Char('L') => self.navigate(Some(shift_month(self.date, 1))),
            KeyCode::Char('r') => self.repeat_picker = Some(RepeatPicker::new(self.repeat)),
            KeyCode::Enter => {
                return CalendarAction::Confirm {
                    date: Some(self.date),
                    repeat: None,
                };
            }
            KeyCode::Esc => {
                return CalendarAction::Cancel;
            }
            _ => {}
        }
        CalendarAction::None
    }

    fn navigate(&mut self, date: Option<Date>) {
        if let Some(date) = date {
            self.date = date;
        }
    }
}

impl Widget for &CalendarPopup {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup = centered_rect(area, 24, 5 + 10 + 3 + 5);

        Clear.render(popup, buf);

        let block = Block::bordered()
            .title(" Due Date ")
            .border_style(Style::default().fg(Color::Cyan));

        let inner = block.inner(popup);
        block.render(popup, buf);

        let mut events = CalendarEventStore::today(Style::default().fg(Color::Yellow).bold());
        events.add(self.date, Style::default().bg(Color::Cyan).fg(Color::Black));

        if let Some(repeat_picker) = &self.repeat_picker {
            repeat_picker.render(inner, buf);
            return;
        }

        let [action_hint, cal_area, action_hint2, nav_hint] = Layout::vertical([
            Constraint::Length(5),
            Constraint::Length(10),
            Constraint::Length(3),
            Constraint::Length(5),
        ])
        .areas(inner);

        Paragraph::new(
            "[x]No Date\n\
            [t]Today\n\
            [y]Yesterday\n\
            [n]Tomorrow",
        )
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .render(action_hint, buf);

        Monthly::new(self.date, events)
            .show_month_header(Style::default().bold())
            .show_weekdays_header(Style::default().fg(Color::DarkGray))
            .render(cal_area, buf);

        Paragraph::new("[r]Repeat\n")
            .block(
                Block::default()
                    .borders(Borders::TOP | Borders::BOTTOM)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .render(action_hint2, buf);

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
