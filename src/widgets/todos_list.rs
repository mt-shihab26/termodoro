use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{List, ListItem, ListState};

use crate::kinds::{page::Page, repeat::Repeat};
use crate::models::todo::Todo;

pub struct TodosListWidget<'a> {
    pub items: &'a [Todo],
    pub page: Page,
    pub color: Color,
}

impl<'a> TodosListWidget<'a> {
    pub fn render(self, frame: &mut Frame, area: Rect, state: &mut ListState) {
        let labels: Vec<String> = self
            .items
            .iter()
            .map(|todo| {
                let check = if todo.done { "[✓]" } else { "[ ]" };
                let repeat_icon = if todo.repeat.is_some() {
                    &format!("{} ", Repeat::icon())
                } else {
                    ""
                };
                let mut label = format!(" {} {}{}", check, repeat_icon, todo.text);
                if let Some(date) = todo.due_date {
                    label.push_str(&format!("  [{}]", date));
                }
                label
            })
            .collect();

        let horizontal_padding = 2;
        let top_padding = 1;
        let padded_area = Rect {
            x: area.x + horizontal_padding,
            y: area.y + top_padding,
            width: area.width.saturating_sub(horizontal_padding * 2),
            height: area.height.saturating_sub(top_padding),
        };

        let items: Vec<ListItem> = if matches!(self.page, Page::History) {
            labels
                .into_iter()
                .map(|label| {
                    ListItem::new(label).style(Style::default().fg(Color::DarkGray).add_modifier(Modifier::CROSSED_OUT))
                })
                .collect()
        } else {
            labels
                .into_iter()
                .zip(self.items.iter())
                .map(|(label, todo)| {
                    let style = if todo.done {
                        Style::default().fg(Color::DarkGray).add_modifier(Modifier::CROSSED_OUT)
                    } else {
                        Style::default().fg(Color::White)
                    };
                    ListItem::new(label).style(style)
                })
                .collect()
        };

        let list = List::new(items)
            .highlight_style(Style::default().fg(self.color).bold())
            .highlight_symbol(">");

        frame.render_stateful_widget(list, padded_area, state);
    }
}
