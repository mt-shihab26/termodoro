use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    style::{Color, Style, Stylize},
    widgets::{Block, Clear, List, ListItem, Paragraph, Widget},
};

use crate::{caches::timer::Stat, models::todo::Todo};

pub struct TodoPickerProps<'a> {
    todos: &'a [(Todo, Stat)],
    cursor: usize,
}

pub enum TodoPickerAction {
    Select(i32),
    Cancel,
    None,
}

pub struct TodoPickerState {
    todos: Vec<Todo>,
    stats: Vec<Stat>,
    cursor: usize,
}

impl TodoPickerState {
    pub fn new(todos: Vec<(Todo, Stat)>) -> Self {
        Self { todos, cursor: 0 }
    }

    pub fn props(&self) -> TodoPickerProps<'_> {
        TodoPickerProps {
            todos: &self.todos,
            cursor: self.cursor,
        }
    }

    pub fn handle(&mut self, key: KeyEvent) -> TodoPickerAction {
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if !self.todos.is_empty() {
                    self.cursor = (self.cursor + 1).min(self.todos.len() - 1);
                }
                TodoPickerAction::None
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.cursor = self.cursor.saturating_sub(1);
                TodoPickerAction::None
            }
            KeyCode::Enter => {
                if let Some((todo, _)) = self.todos.get(self.cursor) {
                    if let Some(id) = todo.id {
                        return TodoPickerAction::Select(id);
                    }
                }
                TodoPickerAction::Cancel
            }
            KeyCode::Esc => TodoPickerAction::Cancel,
            _ => TodoPickerAction::None,
        }
    }
}

pub struct TodoPickerWidget<'a> {
    props: &'a TodoPickerProps<'a>,
}

impl<'a> TodoPickerWidget<'a> {
    pub fn new(props: &'a TodoPickerProps<'a>) -> Self {
        Self { props }
    }
}

impl Widget for &TodoPickerWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let list_height = (self.props.todos.len().min(5) as u16).max(1);
        let popup = centered_rect(area, 52, list_height + 4);

        Clear.render(popup, buf);

        let block = Block::bordered()
            .title(" Select Todo ")
            .title_bottom(" [j/k] ↑↓  [Enter] confirm  [Esc] cancel ")
            .border_style(Style::default().fg(Color::Yellow).bold());

        let inner = block.inner(popup);
        block.render(popup, buf);

        if self.props.todos.is_empty() {
            Paragraph::new("No todos for today")
                .centered()
                .fg(Color::DarkGray)
                .render(inner, buf);
            return;
        }

        let start = self
            .props
            .cursor
            .saturating_sub(2)
            .min(self.props.todos.len().saturating_sub(5));

        let items: Vec<ListItem> = self
            .props
            .todos
            .iter()
            .enumerate()
            .skip(start)
            .take(5)
            .map(|(i, (todo, stat))| {
                let label = if stat.sessions > 0 {
                    format!("{}  ·  {} sessions", todo.text, stat.sessions)
                } else {
                    todo.text.clone()
                };
                if i == self.props.cursor {
                    ListItem::new(format!("> {label}")).style(Style::new().fg(Color::Yellow).bold())
                } else {
                    ListItem::new(format!("  {label}")).style(Style::new().fg(Color::DarkGray))
                }
            })
            .collect();

        List::new(items).render(inner, buf);
    }
}

fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    Rect {
        x: area.x + area.width.saturating_sub(width) / 2,
        y: area.y + area.height.saturating_sub(height) / 2,
        width: width.min(area.width),
        height: height.min(area.height),
    }
}
