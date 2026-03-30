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
        // Height: 8 calendar + 1 nav hint + action row + border(2)
        // +repeat: + 1 separator + options + 1 hint
        let popup_h = if self.repeat_cursor.is_some() {
            8 + 1 + 1 + REPEAT_OPTIONS.len() as u16 + 1 + 2
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

        if let Some(cursor) = self.repeat_cursor {
            let [cal_area, nav_hint, sep, repeat_area, repeat_hint] = Layout::vertical([
                Constraint::Length(8),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(REPEAT_OPTIONS.len() as u16),
                Constraint::Length(1),
            ])
            .areas(inner);

            Monthly::new(self.view, events)
                .show_month_header(Style::default().bold())
                .show_weekdays_header(Style::default().fg(Color::DarkGray))
                .render(cal_area, buf);

            Paragraph::new("[h/l]Day  [j/k]Week  [H/L]Month  [t/y/n]Today/Yest/Tom  [Enter]Confirm")
                .centered()
                .fg(Color::DarkGray)
                .render(nav_hint, buf);

            Paragraph::new("─── Repeat ───")
                .centered()
                .fg(Color::DarkGray)
                .render(sep, buf);

            let items: Vec<ListItem> = REPEAT_OPTIONS
                .iter()
                .enumerate()
                .map(|(i, &opt)| {
                    let active = i == cursor;
                    let style = if active {
                        Style::default().fg(Color::Cyan).bold()
                    } else {
                        Style::default().fg(Color::White)
                    };
                    ListItem::new(format!("{} {}", if active { ">" } else { " " }, opt)).style(style)
                })
                .collect();

            List::new(items).render(repeat_area, buf);

            Paragraph::new("[j/k]Navigate  [Enter]Confirm  [Esc]Back")
                .centered()
                .fg(Color::DarkGray)
                .render(repeat_hint, buf);
        } else {
            let [cal_area, nav_hint, action_hint] =
                Layout::vertical([Constraint::Length(8), Constraint::Length(1), Constraint::Length(1)]).areas(inner);

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
}

fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    Rect {
        x: area.x + area.width.saturating_sub(width) / 2,
        y: area.y + area.height.saturating_sub(height) / 2,
        width: width.min(area.width),
        height: height.min(area.height),
    }
}
