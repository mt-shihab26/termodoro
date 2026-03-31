use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Tabs, Widget};

use crate::kinds::page::Page;

pub struct TodosTabsWidget {
    pub page: Page,
    pub color: Color,
}

impl Widget for TodosTabsWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let tab_titles: Vec<&str> = Page::ALL.iter().map(|p| p.label()).collect();
        let tabs_width: u16 =
            Page::ALL.iter().map(|p| p.label().len() as u16 + 2).sum::<u16>() + (Page::ALL.len() as u16 - 1) * 3;

        let [_, center_area, _] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Length(tabs_width), Constraint::Fill(1)]).areas(area);

        Tabs::new(tab_titles)
            .select(self.page.index())
            .style(Style::default().fg(Color::DarkGray))
            .highlight_style(Style::default().fg(self.color).bold())
            .divider(" | ")
            .render(center_area, buf);
    }
}
