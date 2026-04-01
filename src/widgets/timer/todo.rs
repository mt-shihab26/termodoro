use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::style::Stylize;
use ratatui::widgets::{Paragraph, Widget};

pub struct TodoWidget<'a> {
    pub selected: Option<&'a str>,
    pub stats: Option<(u32, u32)>,
}

impl Widget for &TodoWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = match self.selected {
            Some(text) => match self.stats {
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
