use ratatui::{
    prelude::{Buffer, Color, Rect, Stylize, Widget},
    widgets::Block,
};

/// Props for the outer border widget.
pub struct BorderProps {
    /// Color of the border lines.
    color: Color,
}

impl BorderProps {
    /// Creates new border props with the given border color.
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

/// Widget that renders a colored border and returns the inner area for further rendering.
pub struct BorderWidget<'a> {
    /// Borrowed border props for this render pass.
    props: &'a BorderProps,
    /// Pre-computed inner rect after subtracting the border.
    inner: Rect,
}

impl<'a> BorderWidget<'a> {
    /// Creates a new border widget and computes the inner rect from `area`.
    pub fn new(props: &'a BorderProps, area: Rect) -> Self {
        let inner = Block::bordered().inner(area);
        Self { props, inner }
    }

    /// Renders the border into the buffer and returns the inner rect.
    pub fn render(self, area: Rect, buf: &mut Buffer) -> Rect {
        Block::bordered().fg(self.props.color).render(area, buf);

        self.inner
    }
}
