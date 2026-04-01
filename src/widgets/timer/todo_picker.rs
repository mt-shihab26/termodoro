use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::style::Stylize;
use ratatui::widgets::{List, ListItem, Paragraph, Widget};

pub struct TodoPickerWidget<'a> {
    pub todos: &'a [(i32, String)],
    pub cursor: usize,
}

impl Widget for &TodoPickerWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.todos.is_empty() {
            Paragraph::new("No todos for today")
                .centered()
                .fg(Color::DarkGray)
                .render(area, buf);
            return;
        }

        let start = self
            .cursor
            .saturating_sub(2)
            .min(self.todos.len().saturating_sub(5));

        let items: Vec<ListItem> = self
            .todos
            .iter()
            .enumerate()
            .skip(start)
            .take(5)
            .map(|(i, (_, text))| {
                let is_cursor = i == self.cursor;
                let prefix = if is_cursor { "> " } else { "  " };
                let style = if is_cursor {
                    Style::new().fg(Color::Yellow).bold()
                } else {
                    Style::new().fg(Color::DarkGray)
                };
                ListItem::new(format!("{prefix}{text}")).style(style)
            })
            .collect();

        List::new(items).render(area, buf);
    }
}
