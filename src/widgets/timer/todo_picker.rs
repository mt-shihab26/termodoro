use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    prelude::{Buffer, Color, Rect, Style, Stylize, Widget},
    widgets::{Block, Clear, List, ListItem, Paragraph},
};

use crate::models::{session::Stat, todo::Todo};

/// Action returned by the todo-picker after handling a key event.
pub enum TodoPickerAction {
    /// User confirmed a selection; carries the chosen todo's id.
    Select(i32),
    /// User cancelled the picker without selecting.
    Cancel,
    /// No state change occurred.
    None,
}

/// Props for the todo-picker overlay.
pub struct TodoPickerProps {
    /// Overdue todos (due_date < today).
    due_todos: Vec<Todo>,
    /// Session stats parallel to `due_todos`.
    due_stats: Vec<Stat>,
    /// Today's todos.
    todos: Vec<Todo>,
    /// Session stats parallel to `todos`.
    stats: Vec<Stat>,
    /// Index of the currently highlighted todo (across both sections combined).
    cursor: usize,
    /// Pass the phase color to use everywhere.
    color: Color,
}

impl TodoPickerProps {
    pub fn new(
        due_todos: Vec<Todo>,
        due_stats: Vec<Stat>,
        todos: Vec<Todo>,
        stats: Vec<Stat>,
        color: Color,
    ) -> Self {
        Self {
            due_todos,
            due_stats,
            todos,
            stats,
            cursor: 0,
            color,
        }
    }

    fn total(&self) -> usize {
        self.due_todos.len() + self.todos.len()
    }
}

/// Stateful container for the todo-picker, owns its props and cursor.
pub struct TodoPickerState {
    props: TodoPickerProps,
}

impl TodoPickerState {
    pub fn new(props: TodoPickerProps) -> Self {
        Self { props }
    }

    pub fn props(&self) -> &TodoPickerProps {
        &self.props
    }

    /// Handles a key event and returns the resulting action.
    pub fn handle(&mut self, key: KeyEvent) -> TodoPickerAction {
        let total = self.props.total();
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if total > 0 {
                    self.props.cursor = (self.props.cursor + 1).min(total - 1);
                }
                TodoPickerAction::None
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.props.cursor = self.props.cursor.saturating_sub(1);
                TodoPickerAction::None
            }
            KeyCode::Enter => {
                let due_len = self.props.due_todos.len();
                let cursor = self.props.cursor;
                let id = if cursor < due_len {
                    self.props.due_todos.get(cursor).and_then(|t| t.id)
                } else {
                    self.props.todos.get(cursor - due_len).and_then(|t| t.id)
                };
                if let Some(id) = id {
                    TodoPickerAction::Select(id)
                } else {
                    TodoPickerAction::Cancel
                }
            }
            KeyCode::Esc => TodoPickerAction::Cancel,
            _ => TodoPickerAction::None,
        }
    }
}

/// Stateless widget that renders the todo-picker popup.
pub struct TodoPickerWidget<'a> {
    props: &'a TodoPickerProps,
}

impl<'a> TodoPickerWidget<'a> {
    pub fn new(props: &'a TodoPickerProps) -> Self {
        Self { props }
    }
}

/// A display row: either a non-selectable section header or a selectable todo item.
enum Row {
    Header(&'static str),
    /// logical index into the combined due+today list
    Item(usize),
}

impl Widget for &TodoPickerWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup = centered_rect(area, 60, area.height.saturating_sub(4));

        Clear.render(popup, buf);

        let block = Block::bordered()
            .title(" Select Todo ")
            .title_bottom(" [j/k] ↑↓  [Enter] confirm  [Esc] cancel ")
            .border_style(Style::default().fg(self.props.color).bold());

        let inner = block.inner(popup);
        block.render(popup, buf);

        let total = self.props.total();

        if total == 0 {
            Paragraph::new("No todos for today")
                .centered()
                .fg(Color::DarkGray)
                .render(inner, buf);
            return;
        }

        // Build display rows (headers + items)
        let mut rows: Vec<Row> = vec![];
        if !self.props.due_todos.is_empty() {
            rows.push(Row::Header("Overdue"));
            for i in 0..self.props.due_todos.len() {
                rows.push(Row::Item(i));
            }
        }
        if !self.props.todos.is_empty() {
            rows.push(Row::Header("Today"));
            let due_len = self.props.due_todos.len();
            for i in 0..self.props.todos.len() {
                rows.push(Row::Item(due_len + i));
            }
        }

        // Find which display row the cursor item is on
        let cursor_row = rows.iter().position(|r| matches!(r, Row::Item(i) if *i == self.props.cursor)).unwrap_or(0);

        let visible = inner.height as usize;
        let serial_width = total.max(1).to_string().len();
        let due_len = self.props.due_todos.len();

        // Center scroll around the cursor's display row
        let start = cursor_row
            .saturating_sub(visible / 2)
            .min(rows.len().saturating_sub(visible));

        let items: Vec<ListItem> = rows
            .iter()
            .enumerate()
            .skip(start)
            .take(visible)
            .map(|(_, row)| match row {
                Row::Header(label) => ListItem::new(format!("── {label} ──"))
                    .style(Style::new().fg(Color::DarkGray)),
                Row::Item(logical_idx) => {
                    let (todo, stat) = if *logical_idx < due_len {
                        (&self.props.due_todos[*logical_idx], &self.props.due_stats[*logical_idx])
                    } else {
                        let i = logical_idx - due_len;
                        (&self.props.todos[i], &self.props.stats[i])
                    };
                    let serial = logical_idx + 1;
                    let label = if stat.completed_sessions > 0 {
                        format!(
                            "{}  ·  {}× {}m",
                            todo.text,
                            stat.completed_sessions,
                            stat.completed_secs / 60
                        )
                    } else {
                        todo.text.clone()
                    };
                    if *logical_idx == self.props.cursor {
                        ListItem::new(format!("> {serial:>serial_width$}. {label}"))
                            .style(Style::new().fg(self.props.color).bold())
                    } else {
                        ListItem::new(format!("  {serial:>serial_width$}. {label}"))
                            .style(Style::new().fg(Color::White))
                    }
                }
            })
            .collect();

        List::new(items).render(inner, buf);
    }
}

/// Computes a centered popup rect of the given dimensions within `area`.
fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    Rect {
        x: area.x + area.width.saturating_sub(width) / 2,
        y: area.y + area.height.saturating_sub(height) / 2,
        width: width.min(area.width),
        height: height.min(area.height),
    }
}
