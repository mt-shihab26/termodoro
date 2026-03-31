use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::models::todo::Todo;

use super::todo_item::TodoItemWidget;

pub struct TodosIndexWidget<'a> {
    pub items: &'a [Todo],
    pub selected: usize,
    pub color: Color,
}

impl<'a> TodosIndexWidget<'a> {
    pub fn render(self, frame: &mut Frame, area: Rect) {
        let horizontal_padding = 2;
        let top_padding = 1;
        let padded_area = Rect {
            x: area.x + horizontal_padding,
            y: area.y + top_padding,
            width: area.width.saturating_sub(horizontal_padding * 2),
            height: area.height.saturating_sub(top_padding),
        };

        if padded_area.height == 0 {
            return;
        }

        let (rows, selected_row) = self.rows();
        let visible_rows = padded_area.height as usize;
        let start = if rows.len() <= visible_rows {
            0
        } else {
            selected_row
                .saturating_sub(visible_rows.saturating_sub(1))
                .min(rows.len().saturating_sub(visible_rows))
        };
        let end = (start + visible_rows).min(rows.len());

        frame.render_widget(Paragraph::new(rows[start..end].to_vec()), padded_area);
    }

    fn rows(&self) -> (Vec<Line<'static>>, usize) {
        let mut rows = Vec::new();
        let mut selected_row = 0;
        let mut last_date = None;

        for (index, todo) in self.items.iter().enumerate() {
            if todo.due_date != last_date {
                rows.push(section_line(todo.due_date));
                last_date = todo.due_date;
            }

            if index == self.selected {
                selected_row = rows.len();
            }

            rows.push(TodoItemWidget { todo }.line(index == self.selected, self.color));
        }

        (rows, selected_row)
    }
}

fn section_line(date: Option<time::Date>) -> Line<'static> {
    let label = match date {
        Some(date) => format!(" {date} "),
        None => " No Date ".to_string(),
    };

    Line::from(vec![Span::styled(
        label,
        Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD),
    )])
}
