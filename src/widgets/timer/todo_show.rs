use ratatui::{
    prelude::{Buffer, Rect, Stylize, Widget},
    widgets::Paragraph,
};

use crate::{
    kinds::phase::COLOR,
    models::{session::Stat, todo::Todo},
};

/// Props for the active-todo display bar at the bottom of the timer.
pub struct TodoShowProps<'a> {
    /// The todo currently linked to the timer session, if any.
    todo: Option<&'a Todo>,
    /// Accumulated session statistics for the active todo, if any.
    stat: Option<&'a Stat>,
}

impl<'a> TodoShowProps<'a> {
    /// Creates new todo-show props with an optional todo and its stats.
    pub fn new(todo: Option<&'a Todo>, stat: Option<&'a Stat>) -> Self {
        Self { todo, stat }
    }
}

/// Stateless widget that renders the active todo with session stats.
pub struct TodoShowWidget<'a> {
    /// Borrowed todo-show props for this render pass.
    props: &'a TodoShowProps<'a>,
}

impl<'a> TodoShowWidget<'a> {
    /// Creates a new todo-show widget from the given props.
    pub fn new(props: &'a TodoShowProps<'a>) -> Self {
        Self { props }
    }
}

impl Widget for &TodoShowWidget<'_> {
    /// Renders the todo text (and optional stats) centered into the buffer.
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = match self.props.todo {
            Some(todo) => match self.props.stat {
                Some(stat) => {
                    format!(
                        "{}  ·  {} sessions  ·  {} min",
                        todo.text,
                        stat.completed_sessions,
                        stat.completed_secs / 60
                    )
                }
                None => todo.text.clone(),
            },
            None => "No todo selected  [t] pick".to_string(),
        };
        Paragraph::new(text).centered().fg(COLOR).render(area, buf);
    }
}
