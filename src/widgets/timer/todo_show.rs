use ratatui::{
    prelude::{Buffer, Color, Rect, Stylize, Widget},
    widgets::Paragraph,
};

use crate::{caches::timer::Stat, models::todo::Todo};

pub struct TodoShowProps<'a> {
    todo: Option<&'a Todo>,
    stat: Option<&'a Stat>,
}

impl<'a> TodoShowProps<'a> {
    pub fn new(todo: Option<&'a Todo>, stat: Option<&'a Stat>) -> Self {
        Self { todo, stat }
    }
}

pub struct TodoShowWidget<'a> {
    props: &'a TodoShowProps<'a>,
}

impl<'a> TodoShowWidget<'a> {
    pub fn new(props: &'a TodoShowProps<'a>) -> Self {
        Self { props }
    }
}

impl Widget for &TodoShowWidget<'_> {
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
        Paragraph::new(text).centered().fg(Color::DarkGray).render(area, buf);
    }
}
