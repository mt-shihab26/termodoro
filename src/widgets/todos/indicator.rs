use ratatui::{Frame, layout::Rect, style::Stylize, widgets::Paragraph};

use crate::tabs::todos::COLOR;

pub struct IndicatorWidget {
    pub show_more_above: bool,
    pub show_more_below: bool,
}

impl IndicatorWidget {
    pub fn render(self, frame: &mut Frame, top_area: Rect, bottom_area: Rect) {
        if self.show_more_above && top_area.height > 0 {
            frame.render_widget(Paragraph::new("^ more").fg(COLOR).right_aligned(), top_area);
        }

        if self.show_more_below && bottom_area.height > 0 {
            frame.render_widget(
                Paragraph::new("v more").fg(COLOR).right_aligned(),
                bottom_area,
            );
        }
    }
}
