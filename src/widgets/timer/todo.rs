use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::style::Stylize;
use ratatui::widgets::{Paragraph, Widget};

pub struct TodoProps<'a> {
    selected: Option<&'a str>,
    stats: Option<(u32, u32)>,
}

impl<'a> TodoProps<'a> {
    pub fn new(selected: Option<&'a str>, stats: Option<(u32, u32)>) -> Self {
        Self { selected, stats }
    }
}

pub struct TodoWidget<'a> {
    props: &'a TodoProps<'a>,
}

impl<'a> TodoWidget<'a> {
    pub fn new(props: &'a TodoProps<'a>) -> Self {
        Self { props }
    }
}

impl Widget for &TodoWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = match self.props.selected {
            Some(text) => match self.props.stats {
                Some((count, total_secs)) => {
                    format!("{}  ·  {} sessions  ·  {} min", text, count, total_secs / 60)
                }
                None => text.to_string(),
            },
            None => "No todo selected  [t] pick".to_string(),
        };
        Paragraph::new(text).centered().fg(Color::DarkGray).render(area, buf);
    }
}
