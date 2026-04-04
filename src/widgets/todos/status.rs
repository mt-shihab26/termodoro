use ratatui::{
    prelude::{Buffer, Color, Rect, Stylize, Widget},
    widgets::Paragraph,
};

pub struct StatusProps {
    total: usize,
    from: usize,
    to: usize,
    page: usize,
}

impl StatusProps {
    pub fn new(total: usize, from: usize, to: usize, page: usize) -> Self {
        Self { total, from, to, page }
    }
}

pub struct StatusWidget<'a> {
    props: &'a StatusProps,
}

impl<'a> StatusWidget<'a> {
    pub fn new(props: &'a StatusProps) -> Self {
        Self { props }
    }
}

impl Widget for &StatusWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(format!(
            "Page {} • Range {}-{} • Showing {}/{} items",
            self.props.page,
            self.props.from,
            self.props.to,
            self.props.to - self.props.from,
            self.props.total,
        ))
        .fg(Color::DarkGray)
        .right_aligned()
        .render(
            Rect {
                x: area.x + 1,
                y: area.y,
                width: area.width.saturating_sub(2),
                height: 1,
            },
            buf,
        );
    }
}
