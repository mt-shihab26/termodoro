use ratatui::{
    prelude::{Buffer, Rect, Stylize, Widget},
    widgets::Paragraph,
};

use crate::tabs::todos::COLOR;

pub struct IndicatorWidget {
    pub show_more_above: bool,
    pub show_more_below: bool,
}

impl Widget for &IndicatorWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let horizontal_padding = 2;
        let inner_width = area.width.saturating_sub(horizontal_padding * 2);

        if self.show_more_above {
            let top_area = Rect {
                x: area.x + horizontal_padding,
                y: area.y,
                width: inner_width,
                height: 1,
            };
            Paragraph::new("^ more").fg(COLOR).right_aligned().render(top_area, buf);
        }

        if self.show_more_below && area.height > 0 {
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
