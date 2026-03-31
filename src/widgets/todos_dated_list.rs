use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::Paragraph;
use ratatui::widgets::{List, ListState};

use crate::models::todo::Todo;

use super::todo_item::TodoItemWidget;

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
        let padded_area = Rect {
            x: area.x + horizontal_padding,
            y: area.y + top_padding,
            width: area.width.saturating_sub(horizontal_padding * 2),
            height: area.height.saturating_sub(top_padding + bottom_padding),
        };

        if self.show_more_above {
            frame.render_widget(Paragraph::new("^ more").fg(Color::DarkGray), padded_area);
        }

        if self.show_more_below {
            frame.render_widget(
                Paragraph::new("v more").fg(Color::DarkGray).right_aligned(),
                Rect {
                    x: padded_area.x,
                    y: padded_area.y + padded_area.height.saturating_sub(1),
                    width: padded_area.width,
                    height: 1,
                },
            );
        }

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
