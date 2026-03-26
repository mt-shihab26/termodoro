use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Tabs},
};

use crate::db::Db;
use crate::timer::{CompletedWork, Timer};
use crate::ui::tabs;
use crate::ui::util::unix_now;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveTab {
    Timer,
    Todos,
}

pub struct Shared {
    pub timer: Timer,
    pub db: Db,
    pub active_todo_id: Option<i64>,
    pub message: Option<String>,
}

impl Shared {
    pub fn clear_active_todo(&mut self) {
        self.set_active_todo(None);
    }

    pub fn set_active_todo(&mut self, todo_id: Option<i64>) {
        self.active_todo_id = todo_id;
        let _ = self.db.set_active_todo_id(todo_id);
    }

    pub fn active_todo_label(&self) -> Option<String> {
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

    pub fn record_completed_work(&mut self, completed: Option<CompletedWork>) -> bool {
        let Some(c) = completed else { return false };
        let ended_at = unix_now();
        if let Err(e) = self
            .db
            .insert_work_session(self.active_todo_id, c.duration_secs, ended_at)
        {
            self.message = Some(format!("db: failed to insert session: {e}"));
        }
        true
    }
}

pub struct App {
    pub tab: ActiveTab,
    pub shared: Shared,
    timer_tab: tabs::timer::App,
    todos_tab: tabs::todos::App,
}

impl App {
    pub fn new(timer: Timer, db: Db) -> Self {
        let active_todo_id = db.get_active_todo_id().ok().flatten();

        let mut app = Self {
            tab: ActiveTab::Timer,
            shared: Shared {
                timer,
                db,
                active_todo_id,
                message: None,
            },
            timer_tab: tabs::timer::App::new(),
            todos_tab: tabs::todos::App::new(),
        };

        app.todos_tab.refresh(&mut app.shared);
        app
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Tab => {
                self.tab = match self.tab {
                    ActiveTab::Timer => ActiveTab::Todos,
                    ActiveTab::Todos => ActiveTab::Timer,
                };
                return;
            }
            KeyCode::Char('1') => {
                self.tab = ActiveTab::Timer;
                return;
            }
            KeyCode::Char('2') => {
                self.tab = ActiveTab::Todos;
                return;
            }
            _ => {}
        }

        match self.tab {
            ActiveTab::Timer => self.timer_tab.handle_key(&mut self.shared, key),
            ActiveTab::Todos => self.todos_tab.handle_key(&mut self.shared, key),
        }
    }

    pub fn on_tick(&mut self) {
        let completed = self.shared.timer.tick();
        if self.shared.record_completed_work(completed) {
            self.todos_tab.on_work_logged(&mut self.shared);
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let [header, body] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Fill(1)])
            .areas(area);

        self.render_tabs(frame, header);

        match self.tab {
            ActiveTab::Timer => self.timer_tab.render(&mut self.shared, frame, body),
            ActiveTab::Todos => self.todos_tab.render(&mut self.shared, frame, body),
        }
    }

    fn render_tabs(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
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
}
