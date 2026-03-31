use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Stylize};
use ratatui::widgets::Paragraph;

pub struct TodosOverflowWidget {
    pub show_more_above: bool,
    pub show_more_below: bool,
}

impl TodosOverflowWidget {
    pub fn render(self, frame: &mut Frame, top_area: Rect, bottom_area: Rect) {
        if self.show_more_above && top_area.height > 0 {
            frame.render_widget(Paragraph::new("^ more").fg(Color::DarkGray), top_area);
        }

        if self.show_more_below && bottom_area.height > 0 {
            frame.render_widget(
                Paragraph::new("v more").fg(Color::DarkGray).right_aligned(),
                bottom_area,
            );
        }
    }
}
