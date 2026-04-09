use ratatui::{
    prelude::{Buffer, Rect, Style, Widget},
    widgets::Block,
};
use ratatui_textarea::TextArea;

use crate::tabs::todos::COLOR;

/// Action returned by the search widget after handling a key event.
pub enum SearchAction {
    /// User pressed Enter to confirm; keeps the filter active and closes the bar.
    Confirm,
    /// User pressed Escape to cancel; clears the filter and closes the bar.
    Cancel,
    /// Query text changed; carries the new query string.
    QueryChanged(String),
    /// No state change occurred.
    None,
}

/// Stateful container for the search bar, owns the textarea.
pub struct SearchState {
    /// Textarea holding the current search query as the user types.
    textarea: TextArea<'static>,
}

impl SearchState {
    /// Creates a new `SearchState`, optionally pre-filling with an existing query.
    pub fn new(existing_query: &str) -> Self {
        let mut textarea = TextArea::default();
        if !existing_query.is_empty() {
            textarea.insert_str(existing_query);
        }
        textarea.set_block(
            Block::bordered()
                .title(" Search ")
                .border_style(Style::default().fg(COLOR)),
        );
        textarea.set_cursor_line_style(Style::default());
        Self { textarea }
    }

    /// Returns a reference to the inner textarea for rendering.
    pub fn textarea(&self) -> &TextArea<'static> {
        &self.textarea
    }

    /// Handles a key event and returns the resulting action.
    pub fn handle(&mut self, key: ratatui::crossterm::event::KeyEvent) -> SearchAction {
        use ratatui::crossterm::event::KeyCode;
        match key.code {
            KeyCode::Enter => SearchAction::Confirm,
            KeyCode::Esc => SearchAction::Cancel,
            _ => {
                self.textarea.input(key);
                SearchAction::QueryChanged(self.textarea.lines()[0].clone())
            }
        }
    }
}

/// Stateless widget that renders the search bar textarea.
pub struct SearchWidget<'a> {
    /// Borrowed textarea for this render pass.
    textarea: &'a TextArea<'static>,
}

impl<'a> SearchWidget<'a> {
    /// Creates a new search widget from the given state.
    pub fn new(state: &'a SearchState) -> Self {
        Self { textarea: state.textarea() }
    }
}

impl Widget for &SearchWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Widget::render(self.textarea, area, buf);
    }
}
