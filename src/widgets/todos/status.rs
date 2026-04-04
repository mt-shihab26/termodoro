//! Pagination status bar showing the current page, item range, and total count.

use ratatui::{
    prelude::{Buffer, Color, Rect, Stylize, Widget},
    widgets::Paragraph,
};

/// Props for the todos pagination status bar.
pub struct StatusProps {
    /// Total number of todos across all pages.
    total: usize,
    /// Index of the first item shown on the current page (inclusive).
    from: usize,
    /// Index of the last item shown on the current page (exclusive).
    to: usize,
    /// Current page number (1-based).
    page: usize,
}

impl StatusProps {
    /// Creates new status props with pagination details.
    pub fn new(total: usize, from: usize, to: usize, page: usize) -> Self {
        Self { total, from, to, page }
    }
}

/// Stateless widget that renders the pagination status line.
pub struct StatusWidget<'a> {
    /// Borrowed status props for this render pass.
    props: &'a StatusProps,
}

impl<'a> StatusWidget<'a> {
    /// Creates a new status widget from the given props.
    pub fn new(props: &'a StatusProps) -> Self {
        Self { props }
    }
}

impl Widget for &StatusWidget<'_> {
    /// Renders the page and range summary right-aligned into the buffer.
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
