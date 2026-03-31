use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Stylize};
use ratatui::widgets::Paragraph;

pub struct TodosOverflowWidget {
    pub show_more_above: bool,
    pub show_more_below: bool,
}

impl TodosOverflowWidget {
    pub fn render(self, frame: &mut Frame, area: Rect) {
        if self.show_more_above {
            frame.render_widget(Paragraph::new("^ more").fg(Color::DarkGray), area);
        }

        if self.show_more_below {
            frame.render_widget(
                Paragraph::new("v more").fg(Color::DarkGray).right_aligned(),
                Rect {
                    x: area.x,
                    y: area.y + area.height.saturating_sub(1),
                    width: area.width,
                    height: 1,
                },
            );
        }
    }
}
