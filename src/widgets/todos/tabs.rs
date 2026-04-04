//! Horizontal page-tab navigation bar for switching between todo views.

use ratatui::{
    prelude::{Buffer, Color, Constraint, Layout, Rect, Style, Widget},
    widgets::Tabs,
};

use crate::kinds::page::Page;

/// Props for the page-tabs navigation bar.
pub struct TabsProps {
    /// Currently active page shown as the selected tab.
    page: Page,
    /// Highlight color for the active tab.
    color: Color,
}

impl TabsProps {
    /// Creates new tabs props from the active page and highlight color.
    pub fn new(page: Page, color: Color) -> Self {
        Self { page, color }
    }
}

/// Stateless widget that renders the horizontal page-tab bar.
pub struct TabsWidget<'a> {
    /// Borrowed tabs props for this render pass.
    props: &'a TabsProps,
}

impl<'a> TabsWidget<'a> {
    /// Creates a new tabs widget from the given props.
    pub fn new(props: &'a TabsProps) -> Self {
        Self { props }
    }
}

impl Widget for &TabsWidget<'_> {
    /// Renders the tab bar, highlighting the active page tab.
    fn render(self, area: Rect, buf: &mut Buffer) {
        let tab_titles: Vec<&str> = Page::ALL.iter().map(|p| p.label()).collect();
        let tabs_width: u16 =
            Page::ALL.iter().map(|p| p.label().len() as u16 + 2).sum::<u16>() + (Page::ALL.len() as u16 - 1) * 3;

        let [area, _] = Layout::horizontal([Constraint::Length(tabs_width), Constraint::Fill(1)]).areas(area);

        Tabs::new(tab_titles)
            .select(self.props.page.index())
            .style(Style::default().fg(Color::DarkGray))
            .highlight_style(Style::default().fg(self.props.color).bold())
            .divider(" | ")
            .render(area, buf);
    }
}
