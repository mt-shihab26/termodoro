use ratatui::prelude::{Buffer, Rect, Widget};

pub enum TodoInputAction {
    Confirm,
    Cancel,
    None,
}

#[derive(Clone)]
pub struct TodoInput {}

impl Widget for TodoInput {
    fn render(self, area: Rect, buf: &mut Buffer) {
        todo!()
    }
}
