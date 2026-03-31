use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{List, ListState};

use crate::models::todo::Todo;

use super::todo_item::TodoItemWidget;

pub struct TodosDatedListWidget<'a> {
    pub items: &'a [Todo],
    pub dimmed: bool,
    pub color: Color,
}

impl<'a> TodosDatedListWidget<'a> {
    pub fn render(self, frame: &mut Frame, area: Rect, state: &mut ListState) {
        let horizontal_padding = 2;
        let top_padding = 1;
        let bottom_padding = 1;
        let padded_area = Rect {
            x: area.x + horizontal_padding,
            y: area.y + top_padding,
            width: area.width.saturating_sub(horizontal_padding * 2),
            height: area.height.saturating_sub(top_padding + bottom_padding),
        };

        let items = self
            .items
            .iter()
            .map(|todo| TodoItemWidget { todo }.list_item(self.dimmed))
            .collect::<Vec<_>>();

        let list = List::new(items)
            .highlight_style(Style::default().fg(self.color).bold())
            .highlight_symbol(">");

        frame.render_stateful_widget(list, padded_area, state);
    }
}
