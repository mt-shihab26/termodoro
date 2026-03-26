mod app;
mod component;
mod hints;
mod phase_label;
mod progress_bar;
mod sessions;
mod status;
mod timer_display;
mod title;

use std::io;
use std::time::{Duration, Instant};

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::crossterm::{cursor, execute, terminal};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Tabs},
};
use ratatui::{Terminal, backend::CrosstermBackend};

use chrono::Local;

use crate::db::{Db, Project, TodoFilter, TodoRow};
use crate::state;
use crate::timer::{CompletedWork, Timer};

use app::App;
use component::Component;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ActiveTab {
    Timer,
    Todos,
}

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

struct UiApp {
    timer: Timer,
    db: Db,
    tab: ActiveTab,

    projects: Vec<Project>,
    todos: Vec<TodoRow>,

    focus: Focus,
    sidebar_index: usize,
    todo_index: usize,
    show_completed: bool,

    mode: Mode,
    input: String,
    message: Option<String>,

    active_todo_id: Option<i64>,
}

impl UiApp {
    fn new(timer: Timer, db: Db) -> Self {
        let active_todo_id = db.get_active_todo_id().ok().flatten();
        let mut s = Self {
            timer,
            db,
            tab: ActiveTab::Timer,
            projects: vec![],
            todos: vec![],
            focus: Focus::Sidebar,
            sidebar_index: 0,
            todo_index: 0,
            show_completed: false,
            mode: Mode::Normal,
            input: String::new(),
            message: None,
            active_todo_id,
        };
        s.refresh_projects();
        s.refresh_todos();
        s
    }

    fn refresh_projects(&mut self) {
        match self.db.list_projects() {
            Ok(p) => self.projects = p,
            Err(e) => {
                self.message = Some(format!("db: failed to list projects: {e}"));
                self.projects = vec![];
            }
        }
        self.sidebar_index = self.sidebar_index.min(self.sidebar_len().saturating_sub(1));
    }

    fn refresh_todos(&mut self) {
        let filter = self.current_filter();
        match self.db.list_todos(filter) {
            Ok(t) => self.todos = t,
            Err(e) => {
                self.message = Some(format!("db: failed to list todos: {e}"));
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

    fn active_todo_label(&self) -> Option<String> {
        let id = self.active_todo_id?;
        match self.db.todo_brief(id) {
            Ok(Some(t)) => {
                if let Some(p) = t.project_name {
                    Some(format!("{p} / {}", t.title))
                } else {
                    Some(t.title)
                }
            }
            _ => None,
        }
    }

    fn handle_global_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Tab => {
                self.tab = match self.tab {
                    ActiveTab::Timer => ActiveTab::Todos,
                    ActiveTab::Todos => ActiveTab::Timer,
                };
            }
            KeyCode::Char('1') => self.tab = ActiveTab::Timer,
            KeyCode::Char('2') => self.tab = ActiveTab::Todos,
            _ => {}
        }
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match &self.mode {
            Mode::Normal => {
                self.handle_global_key(key);
                match self.tab {
                    ActiveTab::Timer => self.handle_timer_key(key),
                    ActiveTab::Todos => self.handle_todos_key(key),
                }
            }
            Mode::AddTodo | Mode::AddProject | Mode::EditDueDate { .. } => self.handle_input_key(key),
        }
    }

    fn handle_timer_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(' ') => self.timer.toggle(),
            KeyCode::Char('s') => {
                let completed = self.timer.skip();
                self.on_completed_work(completed);
            }
            KeyCode::Char('r') => self.timer.reset(),
            KeyCode::Char('u') => {
                self.active_todo_id = None;
                let _ = self.db.set_active_todo_id(None);
            }
            _ => {}
        }
    }

    fn handle_todos_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Left => self.focus = Focus::Sidebar,
            KeyCode::Right => self.focus = Focus::List,
            KeyCode::Char('h') => self.focus = Focus::Sidebar,
            KeyCode::Char('l') => self.focus = Focus::List,

            KeyCode::Char('c') => {
                self.show_completed = !self.show_completed;
                self.refresh_todos();
            }

            KeyCode::Char('u') => {
                self.active_todo_id = None;
                let _ = self.db.set_active_todo_id(None);
            }

            KeyCode::Char('a') => {
                self.mode = Mode::AddTodo;
                self.input.clear();
            }
            KeyCode::Char('p') => {
                self.mode = Mode::AddProject;
                self.input.clear();
            }

            _ => match self.focus {
                Focus::Sidebar => self.handle_sidebar_key(key),
                Focus::List => self.handle_todo_list_key(key),
            },
        }
    }

    fn handle_sidebar_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.sidebar_index = self.sidebar_index.saturating_sub(1);
                self.refresh_todos();
                self.todo_index = 0;
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let max = self.sidebar_len().saturating_sub(1);
                self.sidebar_index = (self.sidebar_index + 1).min(max);
                self.refresh_todos();
                self.todo_index = 0;
            }
            _ => {}
        }
    }

    fn handle_todo_list_key(&mut self, key: KeyEvent) {
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
                    self.active_todo_id = Some(todo.id);
                    let _ = self.db.set_active_todo_id(self.active_todo_id);
                }
            }
            KeyCode::Char('x') => {
                if let Some(todo) = self.todos.get(self.todo_index) {
                    let _ = self.db.toggle_todo_completed(todo.id);
                    self.refresh_todos();
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

    fn handle_input_key(&mut self, key: KeyEvent) {
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
                        if let Err(e) = self.db.create_project(text) {
                            self.message = Some(format!("db: failed to create project: {e}"));
                        }
                        self.refresh_projects();
                        self.refresh_todos();
                    }
                    Mode::AddTodo => {
                        if text.is_empty() {
                            self.mode = Mode::Normal;
                            self.input.clear();
                            return;
                        }
                        let (title, due_date) = parse_todo_input(text);
                        let project_id = self.current_project_id_for_new_todo();
                        if let Err(e) = self.db.create_todo(project_id, &title, due_date.as_deref()) {
                            self.message = Some(format!("db: failed to create todo: {e}"));
                        }
                        self.refresh_todos();
                    }
                    Mode::EditDueDate { todo_id } => {
                        let due_date = match parse_due_date_input(text) {
                            Ok(d) => d,
                            Err(msg) => {
                                self.message = Some(msg);
                                return;
                            }
                        };
                        if let Err(e) = self.db.set_todo_due_date(todo_id, due_date.as_deref()) {
                            self.message = Some(format!("db: failed to set due date: {e}"));
                        }
                        self.refresh_todos();
                    }
                    Mode::Normal => {}
                };
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

    fn on_tick(&mut self) {
        let completed = self.timer.tick();
        self.on_completed_work(completed);
    }

    fn on_completed_work(&mut self, completed: Option<CompletedWork>) {
        let Some(c) = completed else { return };
        let ended_at = unix_now();
        if let Err(e) = self
            .db
            .insert_work_session(self.active_todo_id, c.duration_secs, ended_at)
        {
            self.message = Some(format!("db: failed to insert session: {e}"));
        }
        self.refresh_todos();
    }

    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let [header, body] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Fill(1)])
            .areas(area);

        self.render_tabs(frame, header);

        match self.tab {
            ActiveTab::Timer => self.render_timer(frame, body),
            ActiveTab::Todos => self.render_todos(frame, body),
        }
    }

    fn render_tabs(&self, frame: &mut Frame, area: Rect) {
        let titles = vec![Line::from("Timer"), Line::from("Todos")];
        let selected = match self.tab {
            ActiveTab::Timer => 0,
            ActiveTab::Todos => 1,
        };
        frame.render_widget(
            Tabs::new(titles)
                .select(selected)
                .block(Block::default().borders(Borders::ALL).title("termodoro"))
                .style(Style::default().fg(Color::DarkGray))
                .highlight_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            area,
        );
    }

    fn render_timer(&self, frame: &mut Frame, area: Rect) {
        let active = self.active_todo_label();
        let active_ref = active.as_deref();
        App {
            timer: &self.timer,
            active_todo: active_ref,
        }
        .render(frame, area);
    }

    fn render_todos(&mut self, frame: &mut Frame, area: Rect) {
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
        self.render_todo_list(frame, list);
        self.render_todos_footer(frame, footer);
        if input_h > 0 {
            self.render_input(frame, input);
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

    fn render_todo_list(&self, frame: &mut Frame, area: Rect) {
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
                let active = if Some(t.id) == self.active_todo_id { ">" } else { " " };
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

    fn render_todos_footer(&self, frame: &mut Frame, area: Rect) {
        let active = self
            .active_todo_label()
            .map(|s| format!("Active: {s}"))
            .unwrap_or_else(|| "Active: (none)".to_string());
        let msg = self.message.as_deref().unwrap_or(
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

pub fn run(timer: Timer) -> io::Result<()> {
    let db = Db::open().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "failed to open sqlite db"))?;
    let mut app = UiApp::new(timer, db);

    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = event_loop(&mut terminal, &mut app);

    state::save_state(&app.timer.state);

    execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;
    result
}

fn today_string() -> String {
    Local::now().date_naive().format("%Y-%m-%d").to_string()
}

fn unix_now() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

fn parse_todo_input(input: &str) -> (String, Option<String>) {
    // Very small convention: if the last token looks like YYYY-MM-DD, treat it as due_date.
    let trimmed = input.trim();
    if let Some((head, tail)) = trimmed.rsplit_once(' ') {
        if looks_like_date(tail) {
            return (head.trim().to_string(), Some(tail.to_string()));
        }
        if let Ok(Some(d)) = parse_due_date_input(tail) {
            return (head.trim().to_string(), Some(d));
        }
    }
    (trimmed.to_string(), None)
}

fn parse_due_date_input(input: &str) -> Result<Option<String>, String> {
    let t = input.trim();
    if t.is_empty() {
        return Ok(None);
    }
    if t.eq_ignore_ascii_case("none") || t.eq_ignore_ascii_case("clear") {
        return Ok(None);
    }
    if t.eq_ignore_ascii_case("today") {
        return Ok(Some(today_string()));
    }
    if t.eq_ignore_ascii_case("tomorrow") {
        let d = Local::now().date_naive() + chrono::Duration::days(1);
        return Ok(Some(d.format("%Y-%m-%d").to_string()));
    }
    if looks_like_date(t) {
        return Ok(Some(t.to_string()));
    }
    Err("due date: expected YYYY-MM-DD, today, tomorrow, or empty".to_string())
}

fn looks_like_date(s: &str) -> bool {
    let b = s.as_bytes();
    if b.len() != 10 {
        return false;
    }
    b[0..4].iter().all(|c| c.is_ascii_digit())
        && b[4] == b'-'
        && b[5..7].iter().all(|c| c.is_ascii_digit())
        && b[7] == b'-'
        && b[8..10].iter().all(|c| c.is_ascii_digit())
}

fn format_work(secs: u64) -> String {
    let mins = secs / 60;
    let h = mins / 60;
    let m = mins % 60;
    if h > 0 { format!("{h}h{m:02}m") } else { format!("{m}m") }
}

fn event_loop(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut UiApp) -> io::Result<()> {
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|frame| {
            app.render(frame);
        })?;

        let elapsed = last_tick.elapsed();
        let timeout = Duration::from_secs(1).saturating_sub(elapsed);

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match (key.code, key.modifiers) {
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => break,
                    _ => {}
                }
                match key.code {
                    KeyCode::Char('q') => break,
                    _ => {}
                }

                app.handle_key(key);
            }
        }

        if last_tick.elapsed() >= Duration::from_secs(1) {
            app.on_tick();
            last_tick = Instant::now();
        }
    }

    Ok(())
}
