use ratatui::{
    prelude::{Buffer, Color, Rect, Stylize, Widget},
    widgets::Block,
};

pub struct BorderProps {
    color: Color,
}

impl BorderProps {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

pub struct BorderWidget<'a> {
    props: &'a BorderProps,
    inner: Rect,
}

impl<'a> BorderWidget<'a> {
    pub fn new(props: &'a BorderProps, area: Rect) -> Self {
        let inner = Block::bordered().inner(area);
        Self { props, inner }
    }

    pub fn render(self, area: Rect, buf: &mut Buffer) -> Rect {
        Block::bordered().fg(self.props.color).render(area, buf);

        self.inner
    }
}
