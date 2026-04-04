//! Scroll overflow indicator that shows "^ more" or "v more" when items are hidden.

use ratatui::{
    prelude::{Buffer, Rect, Stylize, Widget},
    widgets::Paragraph,
};

use crate::tabs::todos::COLOR;

/// Props for the scroll-overflow indicator widget.
pub struct IndicatorProps {
    /// Whether there are hidden items above the visible window.
    show_more_above: bool,
    /// Whether there are hidden items below the visible window.
    show_more_below: bool,
}

impl IndicatorProps {
    /// Creates new indicator props with above/below overflow flags.
    pub fn new(show_more_above: bool, show_more_below: bool) -> Self {
        Self {
            show_more_above,
            show_more_below,
        }
    }
}

/// Stateless widget that renders "^ more" / "v more" overflow cues.
pub struct IndicatorWidget<'a> {
    /// Borrowed indicator props for this render pass.
    props: &'a IndicatorProps,
}

impl<'a> IndicatorWidget<'a> {
    /// Creates a new indicator widget from the given props.
    pub fn new(props: &'a IndicatorProps) -> Self {
        Self { props }
    }
}

impl Widget for &IndicatorWidget<'_> {
    /// Renders overflow arrows at the top and/or bottom of the area.
    fn render(self, area: Rect, buf: &mut Buffer) {
        let horizontal_padding = 2;
        let inner_width = area.width.saturating_sub(horizontal_padding * 2);

        if self.props.show_more_above {
            let top_area = Rect {
                x: area.x + horizontal_padding,
                y: area.y,
                width: inner_width,
                height: 1,
            };
            Paragraph::new("^ more").fg(COLOR).right_aligned().render(top_area, buf);
        }

        if self.props.show_more_below && area.height > 0 {
            let bottom_area = Rect {
                x: area.x + horizontal_padding,
                y: area.y + area.height.saturating_sub(1),
                width: inner_width,
                height: 1,
            };
            Paragraph::new("v more")
                .fg(COLOR)
                .right_aligned()
                .render(bottom_area, buf);
        }
    }
}
