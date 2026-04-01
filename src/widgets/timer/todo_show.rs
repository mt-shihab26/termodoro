use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::style::Stylize;
use ratatui::widgets::{Paragraph, Widget};

use crate::states::timer_cache::Stat;

pub struct TodoShowProps<'a> {
    selected: Option<&'a str>,
    stats: Option<&'a Stat>,
}

impl<'a> TodoShowProps<'a> {
    pub fn new(selected: Option<&'a str>, stats: Option<&'a Stat>) -> Self {
        Self { selected, stats }
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
        let text = match self.props.selected {
            Some(text) => match self.props.stats {
                Some(stat) => {
                    format!("{}  ·  {} sessions  ·  {} min", text, stat.sessions, stat.secs / 60)
                }
                None => text.to_string(),
            },
            None => "No todo selected  [t] pick".to_string(),
        };
        Paragraph::new(text).centered().fg(Color::DarkGray).render(area, buf);
    }
}
