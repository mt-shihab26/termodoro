use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::style::Stylize;
use ratatui::widgets::{Paragraph, Widget};

pub struct HintProps {
    selecting_todo: bool,
}

impl HintProps {
    pub fn new(selecting_todo: bool) -> Self {
        Self { selecting_todo }
    }
}

pub struct HintWidget<'a> {
    props: &'a HintProps,
}

impl<'a> HintWidget<'a> {
    pub fn new(props: &'a HintProps) -> Self {
        Self { props }
    }
}

impl Widget for &HintWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = if self.props.selecting_todo {
            "[j/k] Navigate   [Enter] Select   [Esc] Cancel"
        } else {
            "[Space] Toggle   [r] Reset   [n] Skip   [t] Todo   [T] Clear todo"
        };
        Paragraph::new(text).centered().fg(Color::DarkGray).render(area, buf);
    }
}
