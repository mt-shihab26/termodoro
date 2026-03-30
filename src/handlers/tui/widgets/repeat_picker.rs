use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{List, ListItem, Paragraph, Widget};

pub const OPTIONS: &[&str] = &[
    "None",
    "Daily",
    "Weekly (same day)",
    "Weekdays (Mon-Fri)",
    "Monthly on day",
    "Yearly on day",
];

/// Renders the repeat option list + its hint line.
/// Height required: `OPTIONS.len() + 1`.
pub struct RepeatPicker {
    pub cursor: usize,
}

impl RepeatPicker {
    pub fn new(cursor: usize) -> Self {
        Self { cursor }
    }

    /// Total rows this widget needs (list + hint).
    pub const fn height() -> u16 {
        OPTIONS.len() as u16 + 1
    }
}

impl Widget for RepeatPicker {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [list_area, hint_area] = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(area);

        let items: Vec<ListItem> = OPTIONS
            .iter()
            .enumerate()
            .map(|(i, &opt)| {
                let active = i == self.cursor;
                let style = if active {
                    Style::default().fg(Color::Cyan).bold()
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(format!("{} {}", if active { ">" } else { " " }, opt)).style(style)
            })
            .collect();

        List::new(items).render(list_area, buf);

        Paragraph::new("[j/k]Navigate  [Enter]Confirm  [Esc]Back")
            .centered()
            .fg(Color::DarkGray)
            .render(hint_area, buf);
    }
}
