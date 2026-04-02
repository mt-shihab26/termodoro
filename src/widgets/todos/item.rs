use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::ListItem;

use crate::kinds::repeat::Repeat;
use crate::models::session::Stat;
use crate::models::todo::Todo;

pub struct ItemWidget<'a> {
    pub todo: &'a Todo,
    pub stats: Option<Stat>,
}

impl<'a> ItemWidget<'a> {
    pub fn label(&self) -> String {
        let check = if self.todo.done { "[✓]" } else { "[ ]" };
        let repeat_icon = if self.todo.repeat.is_some() {
            &format!("{} ", Repeat::icon())
        } else {
            ""
        };

        let mut label = format!("{} {}{}", check, repeat_icon, self.todo.text);

        if let Some(ref stat) = self.stats {
            if stat.completed_sessions > 0 {
                label.push_str(&format!(
                    "  · {}× {}m",
                    stat.completed_sessions,
                    stat.completed_secs / 60
                ));
            }
        }

        if let Some(date) = self.todo.due_date {
            label.push_str(&format!("  [{}]", date));
        }
        label
    }

    pub fn style(&self, dimmed: bool) -> Style {
        if dimmed || self.todo.done {
            Style::default().fg(Color::DarkGray).add_modifier(Modifier::CROSSED_OUT)
        } else {
            Style::default().fg(Color::White)
        }
    }

    pub fn list_item(&self, dimmed: bool, serial: usize, width: usize) -> ListItem<'static> {
        ListItem::new(format!(" {serial:>width$}. {}", self.label())).style(self.style(dimmed))
    }

    pub fn line(&self, selected: bool, color: Color, serial: usize, width: usize) -> Line<'static> {
        let prefix = if selected { "> " } else { "  " };
        let style = if selected {
            self.style(false).fg(color).add_modifier(Modifier::BOLD)
        } else {
            self.style(false)
        };

        Line::from(vec![Span::styled(
            format!("{prefix}{serial:>width$}. {}", self.label()),
            style,
        )])
    }
}
