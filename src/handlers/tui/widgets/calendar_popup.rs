use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::calendar::{CalendarEventStore, Monthly};
use ratatui::widgets::{Block, Clear, Paragraph, Widget};
use time::Date;

pub struct CalendarPopup {
    pub selected: Date,
    pub view: Date,
}

impl CalendarPopup {
    pub fn new(selected: Date, view: Date) -> Self {
        Self { selected, view }
    }
}

impl Widget for CalendarPopup {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup = centered_rect(area, 26, 13);

        Clear.render(popup, buf);

        let block = Block::bordered()
            .title(" Due Date ")
            .border_style(Style::default().fg(Color::Cyan));
        let inner = block.inner(popup);
        block.render(popup, buf);

        let [cal_area, hint1_area, hint2_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1), Constraint::Length(1)]).areas(inner);

        let mut events = CalendarEventStore::today(Style::default().fg(Color::Yellow).bold());
        events.add(self.selected, Style::default().bg(Color::Cyan).fg(Color::Black));

        Monthly::new(self.view, events)
            .show_month_header(Style::default().bold())
            .show_weekdays_header(Style::default().fg(Color::DarkGray))
            .render(cal_area, buf);

        Paragraph::new("[h/l]Day  [j/k]Week  [H/L]Month")
            .centered()
            .fg(Color::DarkGray)
            .render(hint1_area, buf);

        Paragraph::new("[t]Today  [y]Yesterday  [n]Tomorrow  [Enter]OK  [Esc]Skip")
            .centered()
            .fg(Color::DarkGray)
            .render(hint2_area, buf);
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
