use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{List, ListState, Paragraph, StatefulWidget, Widget};
use time::Duration;

use crate::kinds::page::Page;
use crate::models::todo::Todo;
use crate::utils::date::today;

use super::{indicator::IndicatorWidget, item::ItemWidget};

pub struct ListWidget<'a> {
    pub items: &'a [Todo],
    pub stats: &'a [Option<(u32, u32)>],
    pub offset: usize,
    pub page: Page,
    pub selected: usize,
    pub color: Color,
    pub show_more_above: bool,
    pub show_more_below: bool,
}

impl StatefulWidget for &ListWidget<'_> {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut ListState) {
        let horizontal_padding = 2;
        let top_padding = 1;
        let bottom_padding = 1;
        let padded_area = Rect {
            x: area.x + horizontal_padding,
            y: area.y + top_padding,
            width: area.width.saturating_sub(horizontal_padding * 2),
            height: area.height.saturating_sub(top_padding + bottom_padding),
        };

        if padded_area.height == 0 {
            return;
        }

        match self.page {
            Page::Index => self.render_index(padded_area, buf),
            Page::Due | Page::Today | Page::History => self.render_flat(padded_area, buf, state),
        }

        IndicatorWidget {
            show_more_above: self.show_more_above,
            show_more_below: self.show_more_below,
        }
        .render(area, buf);
    }
}

impl ListWidget<'_> {
    fn render_flat(&self, area: Rect, buf: &mut Buffer, state: &mut ListState) {
        let dimmed = matches!(self.page, Page::History);
        let serial_width = (self.offset + self.items.len()).max(1).to_string().len();
        let items = self
            .items
            .iter()
            .enumerate()
            .map(|(index, todo)| {
                let stats = self.stats.get(index).copied().flatten();
                ItemWidget { todo, stats }.list_item(dimmed, self.offset + index + 1, serial_width)
            })
            .collect::<Vec<_>>();

        let list = List::new(items)
            .highlight_style(Style::default().fg(self.color).bold())
            .highlight_symbol(">");

        StatefulWidget::render(list, area, buf, state);
    }

    fn render_index(&self, area: Rect, buf: &mut Buffer) {
        let (rows, selected_row) = self.index_rows(area.width as usize);
        let visible_rows = area.height as usize;
        let start = if rows.len() <= visible_rows {
            0
        } else {
            selected_row
                .saturating_sub(visible_rows.saturating_sub(1))
                .min(rows.len().saturating_sub(visible_rows))
        };
        let end = (start + visible_rows).min(rows.len());

        Paragraph::new(rows[start..end].to_vec()).render(area, buf);
    }

    fn index_rows(&self, width: usize) -> (Vec<Line<'static>>, usize) {
        let mut rows = Vec::new();
        let mut selected_row = 0;
        let mut last_date = None;
        let serial_width = (self.offset + self.items.len()).max(1).to_string().len();

        for (index, todo) in self.items.iter().enumerate() {
            if todo.due_date != last_date {
                rows.push(section_line(todo.due_date, width, self.color));
                last_date = todo.due_date;
            }

            if index == self.selected {
                selected_row = rows.len();
            }

            let stats = self.stats.get(index).copied().flatten();
            rows.push(ItemWidget { todo, stats }.line(
                index == self.selected,
                self.color,
                self.offset + index + 1,
                serial_width,
            ));
        }

        (rows, selected_row)
    }
}

fn section_line(date: Option<time::Date>, width: usize, color: Color) -> Line<'static> {
    let label = match date {
        Some(date) => format!(" {} ", section_label(date)),
        None => " No Date ".to_string(),
    };
    let label_len = label.chars().count();
    let divider_len = width.saturating_sub(label_len);
    let left_len = divider_len / 2;
    let right_len = divider_len.saturating_sub(left_len);

    Line::from(vec![
        Span::styled(
            "─".repeat(left_len),
            Style::default().fg(color).add_modifier(Modifier::DIM),
        ),
        Span::styled(label, Style::default().fg(color).add_modifier(Modifier::BOLD)),
        Span::styled(
            "─".repeat(right_len),
            Style::default().fg(color).add_modifier(Modifier::DIM),
        ),
    ])
}

fn section_label(date: time::Date) -> String {
    let current = today();

    if date == current {
        format!("{date} Today")
    } else if date == current + Duration::days(1) {
        format!("{date} Tomorrow")
    } else if date == current - Duration::days(1) {
        format!("{date} Yesterday")
    } else {
        date.to_string()
    }
}
