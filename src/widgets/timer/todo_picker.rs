use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    style::{Color, Style, Stylize},
    widgets::{Block, Clear, List, ListItem, Paragraph, Widget},
};

use crate::{
    kinds::phase::COLOR,
    models::{session::Stat, todo::Todo},
};

pub struct TodoPickerProps {
    todos: Vec<Todo>,
    stats: Vec<Stat>,
    cursor: usize,
}

impl TodoPickerProps {
    pub fn new(todos: Vec<Todo>, stats: Vec<Stat>) -> Self {
        Self {
            todos,
            stats,
            cursor: 0,
        }
    }
}

pub enum TodoPickerAction {
    Select(i32),
    Cancel,
    None,
}

pub struct TodoPickerState {
    props: TodoPickerProps,
}

impl TodoPickerState {
    pub fn new(todos: Vec<Todo>, stats: Vec<Stat>) -> Self {
        Self {
            props: TodoPickerProps::new(todos, stats),
        }
    }

    pub fn props(&self) -> &TodoPickerProps {
        &self.props
    }

    pub fn handle(&mut self, key: KeyEvent) -> TodoPickerAction {
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if !self.props.todos.is_empty() {
                    self.props.cursor = (self.props.cursor + 1).min(self.props.todos.len() - 1);
                }
                TodoPickerAction::None
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.props.cursor = self.props.cursor.saturating_sub(1);
                TodoPickerAction::None
            }
            KeyCode::Enter => {
                if let Some(id) = self.props.todos.get(self.props.cursor).and_then(|t| t.id) {
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

pub struct TodoPickerWidget<'a> {
    props: &'a TodoPickerProps,
}

impl<'a> TodoPickerWidget<'a> {
    pub fn new(props: &'a TodoPickerProps) -> Self {
        Self { props }
    }
}

impl Widget for &TodoPickerWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup = centered_rect(area, 60, area.height.saturating_sub(4));

        Clear.render(popup, buf);

        let block = Block::bordered()
            .title(" Select Todo ")
            .title_bottom(" [j/k] ↑↓  [Enter] confirm  [Esc] cancel ")
            .border_style(Style::default().fg(COLOR).bold());

        let inner = block.inner(popup);
        block.render(popup, buf);

        if self.props.todos.is_empty() {
            Paragraph::new("No todos for today")
                .centered()
                .fg(Color::DarkGray)
                .render(inner, buf);
            return;
        }

        let visible = inner.height as usize;
        let serial_width = self.props.todos.len().max(1).to_string().len();

        let start = self
            .props
            .cursor
            .saturating_sub(visible / 2)
            .min(self.props.todos.len().saturating_sub(visible));

        let items: Vec<ListItem> = self
            .props
            .todos
            .iter()
            .zip(self.props.stats.iter())
            .enumerate()
            .skip(start)
            .take(visible)
            .map(|(i, (todo, stat))| {
                let serial = i + 1;
                let label = if stat.completed_sessions > 0 {
                    format!("{}  ·  {}× {}m", todo.text, stat.completed_sessions, stat.completed_secs / 60)
                } else {
                    todo.text.clone()
                };
                if i == self.props.cursor {
                    ListItem::new(format!("> {serial:>serial_width$}. {label}")).style(Style::new().fg(COLOR).bold())
                } else {
                    ListItem::new(format!("  {serial:>serial_width$}. {label}")).style(Style::new().fg(Color::White))
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
