use ratatui::prelude::{Buffer, Color, Constraint, Layout, Line, Rect, Span, Stylize, Widget};
use ratatui::widgets::Paragraph;

use crate::widgets::layout::fps::FpsProps;

use super::fps::FpsWidget;

pub struct HeaderProps<'a> {
    fps_props: Option<&'a FpsProps>,
}

impl<'a> HeaderProps<'a> {
    pub fn new(fps_props: Option<&'a FpsProps>) -> Self {
        Self { fps_props }
    }
}

pub struct HeaderWidget<'a> {
    props: &'a HeaderProps<'a>,
}

impl<'a> HeaderWidget<'a> {
    pub fn new(props: &'a HeaderProps<'a>) -> Self {
        Self { props }
    }
}

impl<'a> Widget for &HeaderWidget<'a> {
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

        if self.props.fps_props.is_some() {
            hints.push(Span::from("  ").fg(Color::DarkGray));
            hints.push(Span::from("^f").fg(Color::DarkGray).bold());
            hints.push(Span::from(" fps").fg(Color::DarkGray));
        }

        Line::from(hints).render(left, buf);

        if let Some(fps_props) = self.props.fps_props {
            FpsWidget::new(fps_props).render(right, buf);
        }
    }
}
