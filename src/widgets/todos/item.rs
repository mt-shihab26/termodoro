use ratatui::{
    prelude::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::ListItem,
};

use crate::{
    kinds::repeat::Repeat,
    models::{session::Stat, todo::Todo},
};

pub struct ItemProps<'a> {
    todo: &'a Todo,
    stats: Option<Stat>,
}

impl<'a> ItemProps<'a> {
    pub fn new(todo: &'a Todo, stats: Option<Stat>) -> Self {
        Self { todo, stats }
    }
}

pub struct ItemWidget<'a> {
    props: &'a ItemProps<'a>,
}

impl<'a> ItemWidget<'a> {
    pub fn new(props: &'a ItemProps<'a>) -> Self {
        Self { props }
    }

    pub fn label(&self) -> String {
        let check = if self.props.todo.done { "[✓]" } else { "[ ]" };
        let repeat_icon = if self.props.todo.repeat.is_some() {
            &format!("{} ", Repeat::icon())
        } else {
            ""
        };

        let mut label = format!("{} {}{}", check, repeat_icon, self.props.todo.text);

        if let Some(ref stat) = self.props.stats {
            if stat.completed_sessions > 0 {
                label.push_str(&format!(
                    "  · {}× {}m",
                    stat.completed_sessions,
                    stat.completed_secs / 60
                ));
            }
        }

        if let Some(date) = self.props.todo.due_date {
            label.push_str(&format!("  [{}]", date));
        }
        label
    }

    pub fn style(&self, dimmed: bool) -> Style {
        if dimmed || self.props.todo.done {
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
