use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Paragraph, Widget};

/// A single tab entry: its display name and the effective color (already resolved
/// to `DarkGray` for unselected tabs by the caller).
pub struct TabEntry<'a> {
    pub name: &'a str,
    pub color: Color,
}

pub struct TabsBarWidget<'a> {
    pub tabs: &'a [TabEntry<'a>],
}

impl Widget for &TabsBarWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let constraints = vec![Constraint::Fill(1); self.tabs.len()];
        let tab_areas = Layout::horizontal(constraints).split(area);

        for (entry, &tab_area) in self.tabs.iter().zip(tab_areas.iter()) {
            let block = Block::bordered().border_style(Style::default().fg(entry.color).bold());
            let inner = block.inner(tab_area);
            block.render(tab_area, buf);
            Paragraph::new(entry.name)
                .centered()
                .style(Style::default().fg(entry.color).bold())
                .render(inner, buf);
        }
    }
}
