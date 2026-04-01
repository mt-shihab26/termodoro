use ratatui::prelude::{Alignment, Buffer, Color, Line, Rect, Span, Stylize, Widget};

pub struct FpsProps {
    per_second: f64,
    per_lifetime: u64,
}

impl FpsProps {
    pub fn new(per_second: f64, per_lifetime: u64) -> Self {
        Self {
            per_second,
            per_lifetime,
        }
    }
}

pub struct FpsWidget<'a> {
    props: &'a FpsProps,
}

impl<'a> FpsWidget<'a> {
    pub fn new(props: &'a FpsProps) -> Self {
        Self { props }
    }
}

impl<'a> Widget for &FpsWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Line::from(
            Span::from(format!(
                "{:.0} fps  {} frames",
                self.props.per_second, self.props.per_lifetime
            ))
            .fg(Color::DarkGray),
        )
        .alignment(Alignment::Right)
        .render(area, buf);
    }
}
