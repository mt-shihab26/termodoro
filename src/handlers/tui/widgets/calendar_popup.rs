use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::calendar::{CalendarEventStore, Monthly};
use ratatui::widgets::{Block, Clear, List, ListItem, Paragraph, Widget};
use time::Date;

pub const REPEAT_OPTIONS: &[&str] = &[
    "None",
    "Daily",
    "Weekly (same day)",
    "Weekdays (Mon-Fri)",
    "Monthly on day",
    "Yearly on day",
];

/// Calendar popup. When `repeat_cursor` is `Some`, the repeat list is shown
/// below the calendar in the same popup.
pub struct CalendarPopup {
    pub selected: Date,
    pub view: Date,
    /// `None` = calendar only; `Some(n)` = repeat section visible, cursor at n.
    pub repeat_cursor: Option<usize>,
}

impl CalendarPopup {
    pub fn new(selected: Date, view: Date) -> Self {
        Self {
            selected,
            view,
            repeat_cursor: None,
        }
    }

    pub fn with_repeat(selected: Date, view: Date, cursor: usize) -> Self {
        Self {
            selected,
            view,
            repeat_cursor: Some(cursor),
        }
    }
}

impl Widget for CalendarPopup {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Always show repeat options below the calendar.
        // Height: 8 cal + 1 nav hint + 1 action hint + 1 sep + options + 1 repeat hint + 2 border
        let popup_h = 8 + 1 + 1 + 1 + REPEAT_OPTIONS.len() as u16 + 1 + 2;

        let popup = centered_rect(area, 28, popup_h);
        Clear.render(popup, buf);

        let block = Block::bordered()
            .title(" Due Date ")
            .border_style(Style::default().fg(Color::Cyan));
        let inner = block.inner(popup);
        block.render(popup, buf);

        let [cal_area, nav_hint, action_hint, sep, repeat_area, repeat_hint] = Layout::vertical([
            Constraint::Length(8),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(REPEAT_OPTIONS.len() as u16),
            Constraint::Length(1),
        ])
        .areas(inner);

        let mut events = CalendarEventStore::today(Style::default().fg(Color::Yellow).bold());
        events.add(self.selected, Style::default().bg(Color::Cyan).fg(Color::Black));

        Monthly::new(self.view, events)
            .show_month_header(Style::default().bold())
            .show_weekdays_header(Style::default().fg(Color::DarkGray))
            .render(cal_area, buf);

        let repeat_focused = self.repeat_cursor.is_some();

        Paragraph::new("[h/l]Day  [j/k]Week  [H/L]Month")
            .centered()
            .fg(if repeat_focused { Color::DarkGray } else { Color::Gray })
            .render(nav_hint, buf);

        Paragraph::new(if repeat_focused {
            "[Enter]Confirm date  [Esc]Cancel"
        } else {
            "[t]Today  [y]Yesterday  [n]Tomorrow  [Enter]Confirm  [Esc]Cancel"
        })
        .centered()
        .fg(if repeat_focused { Color::DarkGray } else { Color::Gray })
        .render(action_hint, buf);

        Paragraph::new("─── Repeat ───")
            .centered()
            .fg(if repeat_focused { Color::Cyan } else { Color::DarkGray })
            .render(sep, buf);

        let cursor = self.repeat_cursor.unwrap_or(usize::MAX); // no highlight when unfocused
        let items: Vec<ListItem> = REPEAT_OPTIONS
            .iter()
            .enumerate()
            .map(|(i, &opt)| {
                let active = repeat_focused && i == cursor;
                let style = if active {
                    Style::default().fg(Color::Cyan).bold()
                } else if repeat_focused {
                    Style::default().fg(Color::White)
                } else {
                    Style::default().fg(Color::DarkGray)
                };
                ListItem::new(format!("{} {}", if active { ">" } else { " " }, opt)).style(style)
            })
            .collect();

        List::new(items).render(repeat_area, buf);

        Paragraph::new(if repeat_focused {
            "[j/k]Navigate  [Enter]Confirm  [Esc]Back"
        } else {
            "[r] Select repeat"
        })
        .centered()
        .fg(if repeat_focused { Color::Gray } else { Color::DarkGray })
        .render(repeat_hint, buf);
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
