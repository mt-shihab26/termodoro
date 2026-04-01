use ratatui::prelude::{Buffer, Color, Constraint, Layout, Line, Rect, Span, Stylize, Widget};
use ratatui::widgets::Paragraph;

use crate::widgets::layout::fps::FpsProps;

use super::fps::FpsWidget;

pub struct HeaderWidget {
    fps_show: bool,
    fps_props: FpsProps,
}

impl HeaderWidget {
    pub fn new(fps_show: bool, fps_props: FpsProps) -> Self {
        Self {
            fps_show,
            fps_props,
        }
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

        if self.fps_show {
            hints.push(Span::from("  ").fg(Color::DarkGray));
            hints.push(Span::from("^f").fg(Color::DarkGray).bold());
            hints.push(Span::from(" fps").fg(Color::DarkGray));
        }

        Line::from(hints).render(left, buf);

        if self.fps_show {
            FpsWidget::new(self.fps_props).render(right, buf);
        }
    }
}
