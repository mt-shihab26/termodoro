use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::db::{Project, TodoFilter, TodoRow};
use crate::ui::app::Shared;
use crate::ui::util::{format_work, parse_due_date_input, parse_todo_input, today_string};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Focus {
    Sidebar,
    List,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Mode {
    Normal,
    AddTodo,
    AddProject,
    EditDueDate { todo_id: i64 },
}

pub struct App {
    projects: Vec<Project>,
    todos: Vec<TodoRow>,

    focus: Focus,
    sidebar_index: usize,
    todo_index: usize,
    show_completed: bool,

    mode: Mode,
    input: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            projects: vec![],
            todos: vec![],
            focus: Focus::Sidebar,
            sidebar_index: 0,
            todo_index: 0,
            show_completed: false,
            mode: Mode::Normal,
            input: String::new(),
        }
    }

    pub fn refresh(&mut self, shared: &mut Shared) {
        self.refresh_projects(shared);
        self.refresh_todos(shared);
    }

    pub fn on_work_logged(&mut self, shared: &mut Shared) {
        let _ = shared;
        self.refresh_todos(shared);
    }

    pub fn handle_key(&mut self, shared: &mut Shared, key: KeyEvent) {
        match &self.mode {
            Mode::Normal => self.handle_normal_key(shared, key),
            Mode::AddTodo | Mode::AddProject | Mode::EditDueDate { .. } => self.handle_input_key(shared, key),
        }
    }

    pub fn render(&mut self, shared: &mut Shared, frame: &mut Frame, area: Rect) {
        let input_h = if matches!(self.mode, Mode::Normal) { 0 } else { 3 };
        let [body, footer, input] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Length(1), Constraint::Length(input_h)])
            .areas(area);

        let [sidebar, list] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(28), Constraint::Fill(1)])
            .areas(body);

        self.render_sidebar(frame, sidebar);
        self.render_todo_list(shared, frame, list);
        self.render_footer(shared, frame, footer);
        if input_h > 0 {
            self.render_input(frame, input);
        }
    }

    fn handle_normal_key(&mut self, shared: &mut Shared, key: KeyEvent) {
        match key.code {
            KeyCode::Left => self.focus = Focus::Sidebar,
            KeyCode::Right => self.focus = Focus::List,
            KeyCode::Char('h') => self.focus = Focus::Sidebar,
            KeyCode::Char('l') => self.focus = Focus::List,

            KeyCode::Char('c') => {
                self.show_completed = !self.show_completed;
                self.refresh_todos(shared);
            }

            KeyCode::Char('u') => shared.clear_active_todo(),

            KeyCode::Char('a') => {
                self.mode = Mode::AddTodo;
                self.input.clear();
            }
            KeyCode::Char('p') => {
                self.mode = Mode::AddProject;
                self.input.clear();
            }

            _ => match self.focus {
                Focus::Sidebar => self.handle_sidebar_key(shared, key),
                Focus::List => self.handle_list_key(shared, key),
            },
        }
    }

    fn handle_sidebar_key(&mut self, shared: &mut Shared, key: KeyEvent) {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.sidebar_index = self.sidebar_index.saturating_sub(1);
                self.refresh_todos(shared);
                self.todo_index = 0;
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let max = self.sidebar_len().saturating_sub(1);
                self.sidebar_index = (self.sidebar_index + 1).min(max);
                self.refresh_todos(shared);
                self.todo_index = 0;
            }
            _ => {}
        }
    }

    fn handle_list_key(&mut self, shared: &mut Shared, key: KeyEvent) {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.todo_index = self.todo_index.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if !self.todos.is_empty() {
                    self.todo_index = (self.todo_index + 1).min(self.todos.len() - 1);
                }
            }
            KeyCode::Enter => {
                if let Some(todo) = self.todos.get(self.todo_index) {
                    shared.set_active_todo(Some(todo.id));
                }
            }
            KeyCode::Char('x') => {
                if let Some(todo) = self.todos.get(self.todo_index) {
                    if let Err(e) = shared.db.toggle_todo_completed(todo.id) {
                        shared.message = Some(format!("db: failed to toggle todo: {e}"));
                    }
                    self.refresh_todos(shared);
                }
            }
            KeyCode::Char('d') => {
                if let Some(todo) = self.todos.get(self.todo_index) {
                    self.mode = Mode::EditDueDate { todo_id: todo.id };
                    self.input = todo.due_date.clone().unwrap_or_else(today_string);
                }
            }
            _ => {}
        }
    }

    fn handle_input_key(&mut self, shared: &mut Shared, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Normal;
                self.input.clear();
            }
            KeyCode::Enter => {
                let text = self.input.trim();
                match self.mode.clone() {
                    Mode::AddProject => {
                        if text.is_empty() {
                            self.mode = Mode::Normal;
                            self.input.clear();
                            return;
                        }
                        if let Err(e) = shared.db.create_project(text) {
                            shared.message = Some(format!("db: failed to create project: {e}"));
                        }
                        self.refresh_projects(shared);
                        self.refresh_todos(shared);
                    }
                    Mode::AddTodo => {
                        if text.is_empty() {
                            self.mode = Mode::Normal;
                            self.input.clear();
                            return;
                        }
                        let (title, due_date) = parse_todo_input(text);
                        let project_id = self.current_project_id_for_new_todo();
                        if let Err(e) = shared.db.create_todo(project_id, &title, due_date.as_deref()) {
                            shared.message = Some(format!("db: failed to create todo: {e}"));
                        }
                        self.refresh_todos(shared);
                    }
                    Mode::EditDueDate { todo_id } => {
                        let due_date = match parse_due_date_input(text) {
                            Ok(d) => d,
                            Err(msg) => {
                                shared.message = Some(msg);
                                return;
                            }
                        };
                        if let Err(e) = shared.db.set_todo_due_date(todo_id, due_date.as_deref()) {
                            shared.message = Some(format!("db: failed to set due date: {e}"));
                        }
                        self.refresh_todos(shared);
                    }
                    Mode::Normal => {}
                }

                self.mode = Mode::Normal;
                self.input.clear();
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Char(c) => {
                if !key.modifiers.contains(KeyModifiers::CONTROL) {
                    self.input.push(c);
                }
            }
            _ => {}
        }
    }

    fn refresh_projects(&mut self, shared: &mut Shared) {
        match shared.db.list_projects() {
            Ok(p) => self.projects = p,
            Err(e) => {
                shared.message = Some(format!("db: failed to list projects: {e}"));
                self.projects = vec![];
            }
        }
        self.sidebar_index = self.sidebar_index.min(self.sidebar_len().saturating_sub(1));
    }

    fn refresh_todos(&mut self, shared: &mut Shared) {
        let filter = self.current_filter();
        match shared.db.list_todos(filter) {
            Ok(t) => self.todos = t,
            Err(e) => {
                shared.message = Some(format!("db: failed to list todos: {e}"));
                self.todos = vec![];
            }
        }
        self.todo_index = self.todo_index.min(self.todos.len().saturating_sub(1));
    }

    fn sidebar_len(&self) -> usize {
        2 + self.projects.len()
    }

    fn current_filter(&self) -> TodoFilter {
        if self.sidebar_index == 0 {
            TodoFilter::Today {
                date: today_string(),
                show_completed: self.show_completed,
            }
        } else if self.sidebar_index == 1 {
            TodoFilter::Index {
                show_completed: self.show_completed,
            }
        } else {
            let project_index = self.sidebar_index.saturating_sub(2);
            let project_id = self.projects.get(project_index).map(|p| p.id).unwrap_or(-1);
            TodoFilter::Project {
                project_id,
                show_completed: self.show_completed,
            }
        }
    }

    fn current_project_id_for_new_todo(&self) -> Option<i64> {
        if self.sidebar_index >= 2 {
            let project_index = self.sidebar_index - 2;
            self.projects.get(project_index).map(|p| p.id)
        } else {
            None
        }
    }

    fn render_sidebar(&self, frame: &mut Frame, area: Rect) {
        let mut items: Vec<ListItem> = Vec::with_capacity(self.sidebar_len());
        items.push(ListItem::new("Today"));
        items.push(ListItem::new("Index"));
        for p in &self.projects {
            items.push(ListItem::new(p.name.clone()));
        }

        let title = if self.show_completed {
            "Views (incl. done)"
        } else {
            "Views"
        };
        let block = Block::default().borders(Borders::ALL).title(title);
        let highlight = if self.focus == Focus::Sidebar {
            Style::default().fg(Color::White).bg(Color::Blue)
        } else {
            Style::default().fg(Color::Cyan)
        };

        let mut state = ListState::default();
        state.select(Some(self.sidebar_index));
        frame.render_stateful_widget(
            List::new(items)
                .block(block)
                .highlight_style(highlight)
                .highlight_symbol("> "),
            area,
            &mut state,
        );
    }

    fn render_todo_list(&self, shared: &mut Shared, frame: &mut Frame, area: Rect) {
        let title = match self.current_filter() {
            TodoFilter::Today { .. } => format!("Today ({})", today_string()),
            TodoFilter::Index { .. } => "Index".to_string(),
            TodoFilter::Project { project_id, .. } => self
                .projects
                .iter()
                .find(|p| p.id == project_id)
                .map(|p| p.name.clone())
                .unwrap_or_else(|| "Project".to_string()),
        };

        let items: Vec<ListItem> = self
            .todos
            .iter()
            .map(|t| {
                let active = if Some(t.id) == shared.active_todo_id { ">" } else { " " };
                let check = if t.completed_at.is_some() { "[x]" } else { "[ ]" };
                let due = t.due_date.as_deref().map(|d| format!(" {d}")).unwrap_or_default();
                let work = if t.work_secs > 0 {
                    format!("  {}", format_work(t.work_secs))
                } else {
                    String::new()
                };
                let line = format!("{active} {check} {}{due}{work}", t.title);
                ListItem::new(line)
            })
            .collect();

        let highlight = if self.focus == Focus::List {
            Style::default().fg(Color::White).bg(Color::Blue)
        } else {
            Style::default().fg(Color::Cyan)
        };

        let mut state = ListState::default();
        state.select(Some(self.todo_index));
        frame.render_stateful_widget(
            List::new(items)
                .block(Block::default().borders(Borders::ALL).title(title))
                .highlight_style(highlight)
                .highlight_symbol("▸ "),
            area,
            &mut state,
        );
    }

    fn render_footer(&self, shared: &mut Shared, frame: &mut Frame, area: Rect) {
        let active = shared
            .active_todo_label()
            .map(|s| format!("Active: {s}"))
            .unwrap_or_else(|| "Active: (none)".to_string());
        let msg = shared.message.as_deref().unwrap_or(
            "[a] add todo  [p] add project  [d] due date  [x] toggle done  [enter] set active  [c] show done  [h/l] focus",
        );

        frame.render_widget(
            Paragraph::new(format!("{active}  |  {msg}"))
                .style(Style::default().fg(Color::DarkGray))
                .alignment(Alignment::Left),
            area,
        );
    }

    fn render_input(&self, frame: &mut Frame, area: Rect) {
        let (title, hint) = match self.mode {
            Mode::AddTodo => ("New Todo", "Enter: add  Esc: cancel  Tip: append YYYY-MM-DD"),
            Mode::AddProject => ("New Project", "Enter: add  Esc: cancel"),
            Mode::EditDueDate { .. } => (
                "Due Date",
                "Enter: save  Esc: cancel  Use YYYY-MM-DD, today, tomorrow, or empty to clear",
            ),
            Mode::Normal => ("", ""),
        };

        frame.render_widget(
            Paragraph::new(format!("{}\n{}", &self.input, hint))
                .block(Block::default().borders(Borders::ALL).title(title)),
            area,
        );
    }
}
