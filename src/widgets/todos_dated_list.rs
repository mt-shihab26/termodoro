use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{List, ListState};

use crate::models::todo::Todo;

use super::{todo_item::TodoItemWidget, todos_overflow::TodosOverflowWidget};

pub struct TodosDatedListWidget<'a> {
    pub items: &'a [Todo],
    pub dimmed: bool,
    pub color: Color,
    pub show_more_above: bool,
    pub show_more_below: bool,
}

impl<'a> TodosDatedListWidget<'a> {
    pub fn render(self, frame: &mut Frame, area: Rect, state: &mut ListState) {
        let horizontal_padding = 2;
        let top_padding = 1;
        let bottom_padding = 1;
        let top_indicator_area = Rect {
            x: area.x + horizontal_padding,
            y: area.y,
            width: area.width.saturating_sub(horizontal_padding * 2),
            height: top_padding,
        };
        let bottom_indicator_area = Rect {
            x: area.x + horizontal_padding,
            y: area.y + area.height.saturating_sub(bottom_padding),
            width: area.width.saturating_sub(horizontal_padding * 2),
            height: bottom_padding,
        };
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

        TodosOverflowWidget {
            show_more_above: self.show_more_above,
            show_more_below: self.show_more_below,
        }
        .render(frame, top_indicator_area, bottom_indicator_area);
    }
}
