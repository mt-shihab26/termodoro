use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    prelude::{Buffer, Rect, Stylize, Widget},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::tabs::todos::COLOR;

/// Action returned by the search state after handling a key event.
pub enum SearchAction {
    /// User pressed Enter; keeps the filter active and closes the bar.
    Confirm,
    /// User pressed Escape; clears the filter and closes the bar.
    Cancel,
    /// Query text changed; carries the new query string.
    QueryChanged(String),
    /// No state change occurred.
    None,
}

/// Props for the search widget.
pub struct SearchProps {
    /// The current search query string.
    pub query: String,
    /// Whether the search bar is actively being typed into (shows block cursor).
    pub active: bool,
}

impl SearchProps {
    /// Creates new search props.
    pub fn new(query: impl Into<String>, active: bool) -> Self {
        Self {
            query: query.into(),
            active,
        }
    }
}

/// Stateful container for the search bar; owns the query and handles key events.
pub struct SearchState {
    props: SearchProps,
}

impl SearchState {
    /// Creates a new `SearchState`, optionally pre-filling with an existing query.
    pub fn new(existing_query: &str) -> Self {
        Self {
            props: SearchProps::new(existing_query, true),
        }
    }

    /// Returns a reference to the props for rendering.
    pub fn props(&self) -> &SearchProps {
        &self.props
    }

    /// Handles a key event and returns the resulting action.
    pub fn handle(&mut self, key: KeyEvent) -> SearchAction {
        match key.code {
            KeyCode::Enter => SearchAction::Confirm,
            KeyCode::Esc => SearchAction::Cancel,
            KeyCode::Backspace => {
                self.props.query.pop();
                SearchAction::QueryChanged(self.props.query.clone())
            }
            KeyCode::Char(c) => {
                self.props.query.push(c);
                SearchAction::QueryChanged(self.props.query.clone())
            }
            _ => SearchAction::None,
        }
    }
}

/// Stateless widget that renders the search line as `/query` with an optional block cursor.
pub struct SearchWidget<'a> {
    props: &'a SearchProps,
}

impl<'a> SearchWidget<'a> {
    /// Creates a new search widget from the given props.
    pub fn new(props: &'a SearchProps) -> Self {
        Self { props }
    }
}

impl Widget for &SearchWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut spans = vec![
            Span::from("/").fg(COLOR).bold(),
            Span::from(self.props.query.as_str()).fg(COLOR),
        ];
        if self.props.active {
            spans.push(Span::from("█").fg(COLOR));
        }
        Paragraph::new(Line::from(spans)).render(area, buf);
    }
}
