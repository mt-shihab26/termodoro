use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget};

use crate::widgets::fps::FpsWidget;

pub struct HeaderWidget {
    pub fps_widget: Option<FpsWidget>,
}

impl HeaderWidget {
    pub fn new(fps_widget: Option<FpsWidget>) -> Self {
        Self { fps_widget }
    }
}

impl Widget for &HeaderWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(Span::from("Orivo").bold().fg(Color::Green))
            .centered()
            .render(area, buf);

        let [left, right] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(area);

        let mut hints = vec![
            Span::from("^q").fg(Color::DarkGray).bold(),
            Span::from(" quit").fg(Color::DarkGray),
        ];

        if self.show_fps {
            hints.push(Span::from("  ").fg(Color::DarkGray));
            hints.push(Span::from("^f").fg(Color::DarkGray).bold());
            hints.push(Span::from(" fps").fg(Color::DarkGray));
        }

        Line::from(hints).render(left, buf);

        if let Some(fps_widget) = &mut self.fps_widget {
            fps_widget.render(fps_area2(top), frame.buffer_mut());
        }
    }
}
