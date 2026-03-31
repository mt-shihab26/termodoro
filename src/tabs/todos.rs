use std::cell::RefCell;
use std::io::Result;

use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::widgets::{Block, List, ListItem, ListState, Paragraph, Tabs, Widget};
use sea_orm::DatabaseConnection;
use time::OffsetDateTime;

use crate::kinds::{page::Page, repeat::Repeat};
use crate::log_warn;
use crate::models::todo::Todo;
use crate::widgets::input::{InputAction, InputWidget};

use super::Tab;

pub const COLOR: Color = Color::Green;

pub enum UiMode {
    Normal,
    Adding,
    Editing,
}

pub struct Todos {
    db: DatabaseConnection,
    items: Vec<Todo>,
    page: Page,
    ui_mode: UiMode,
    selected: usize,
    list_state: RefCell<ListState>,
    input_widget: Option<InputWidget>,
}

impl Todos {
    pub fn new(db: DatabaseConnection) -> Self {
        let items = Todo::all(&db);

        Self {
            db,
            items,
            page: Page::Today,
            ui_mode: UiMode::Normal,
            selected: 0,
            list_state: RefCell::new(ListState::default()),
            input_widget: None,
        }
    }

    fn sync_list_state(&self) {
        let len = self.filtered_indices().len();
        let selected = if len == 0 {
            None
        } else {
            Some(self.selected.min(len - 1))
        };
        self.list_state.borrow_mut().select(selected);
    }

    fn filtered_indices(&self) -> Vec<usize> {
        let today = OffsetDateTime::now_utc().date();

        self.items
            .iter()
            .enumerate()
            .filter(|(_, todo)| match self.page {
                Page::Due => todo.due_date.map_or(false, |d| d < today) && !todo.done,
                Page::Today => todo.due_date.map_or(false, |d| d == today),
                Page::Index => todo.due_date.map_or(true, |d| d > today),
                Page::History => todo.done,
            })
            .map(|(i, _)| i)
            .collect()
    }

    fn clamp_selected(&mut self) {
        let len = self.filtered_indices().len();
        if len == 0 {
            self.selected = 0;
        } else {
            self.selected = self.selected.min(len - 1);
        }
    }
}

impl Tab for Todos {
    fn name(&self) -> &str {
        "Todos [^t]"
    }

    fn color(&self) -> Color {
        COLOR
    }

    fn handle(&mut self, key: KeyEvent) -> Result<()> {
        match self.ui_mode {
            UiMode::Normal => match key.code {
                KeyCode::Char(']') => {
                    self.page = self.page.next();
                    self.selected = 0;
                }
                KeyCode::Char('[') => {
                    self.page = self.page.prev();
                    self.selected = 0;
                }
                KeyCode::Char(' ') | KeyCode::Enter => {
                    if let Some(&index) = self.filtered_indices().get(self.selected) {
                        let item = &mut self.items[index];
                        if let Some(next) = item.toggle(&self.db) {
                            self.items.push(next);
                        }
                    }
                }
                KeyCode::Char('j') | KeyCode::Down => {
                    let len = self.filtered_indices().len();
                    if len > 0 {
                        self.selected = (self.selected + 1).min(len - 1);
                    }
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    self.selected = self.selected.saturating_sub(1);
                }
                KeyCode::Char('d') => {
                    if !matches!(self.page, Page::History) {
                        let indices = self.filtered_indices();
                        if let Some(&real) = indices.get(self.selected) {
                            if self.items[real].id.is_none() {
                                log_warn!("todo has no id, skipping db delete");
                            } else {
                                self.items[real].delete(&self.db);
                            }
                            self.items.remove(real);
                            self.clamp_selected();
                        }
                    }
                }
                KeyCode::Char('a') => {
                    if !matches!(self.page, Page::History) {
                        self.ui_mode = UiMode::Adding;
                        self.input_widget = Some(InputWidget::new(None, None, None));
                    }
                }
                KeyCode::Char('e') => {
                    if !matches!(self.page, Page::History) {
                        let indices = self.filtered_indices();
                        if let Some(&real) = indices.get(self.selected) {
                            self.ui_mode = UiMode::Editing;
                            let todo = &self.items[real];
                            self.input_widget =
                                Some(InputWidget::new(Some(&todo.text), todo.due_date, todo.repeat.as_ref()));
                        }
                    }
                }
                KeyCode::Char(c) => {
                    if Page::ALL.iter().any(|p| p.key() == c) {
                        self.page = match c {
                            '1' => Page::Due,
                            '2' => Page::Today,
                            '3' => Page::Index,
                            '4' => Page::History,
                            _ => self.page.next().prev(),
                        };
                        self.selected = 0;
                    }
                }
                _ => {}
            },
            UiMode::Adding => {
                if let Some(input_widget) = &mut self.input_widget {
                    match input_widget.handle(key) {
                        InputAction::Confirm { text, date, repeat } => {
                            let mut todo = Todo::new(text, date, repeat);
                            if todo.save(&self.db) {
                                self.items.push(todo);
                            }
                            self.input_widget = None;
                            self.ui_mode = UiMode::Normal;
                            self.clamp_selected();
                        }
                        InputAction::Escape => {
                            self.input_widget = None;
                            self.ui_mode = UiMode::Normal;
                        }
                        InputAction::None => {}
                    }
                }
            }
            UiMode::Editing => {
                if let Some(input_widget) = &mut self.input_widget {
                    match input_widget.handle(key) {
                        InputAction::Confirm { text, date, repeat } => {
                            let indices = self.filtered_indices();
                            if let Some(&real) = indices.get(self.selected) {
                                {
                                    let todo = &mut self.items[real];
                                    todo.text = text;
                                    todo.due_date = date;
                                    todo.repeat = repeat;
                                    todo.update(&self.db);
                                }
                            }
                            self.input_widget = None;
                            self.ui_mode = UiMode::Normal;
                        }
                        InputAction::Escape => {
                            self.input_widget = None;
                            self.ui_mode = UiMode::Normal;
                        }
                        InputAction::None => {}
                    }
                }
            }
        }

        self.sync_list_state();

        Ok(())
    }

    fn render(&self, frame: &mut Frame, area: Rect) {
        let buf = frame.buffer_mut();

        let block = Block::bordered().fg(self.color());
        let inner = block.inner(area);
        block.render(area, buf);

        let area = inner;

        let (tabs_area, list_area, hint_area, input_area) = match self.ui_mode {
            UiMode::Normal => {
                let [tabs, list, hint] =
                    Layout::vertical([Constraint::Length(1), Constraint::Fill(1), Constraint::Length(1)]).areas(area);
                (tabs, list, hint, None)
            }
            UiMode::Adding | UiMode::Editing => {
                let [tabs, list, hint, input] = Layout::vertical([
                    Constraint::Length(1),
                    Constraint::Fill(1),
                    Constraint::Length(1),
                    Constraint::Length(3),
                ])
                .areas(area);
                (tabs, list, hint, Some(input))
            }
        };

        let tab_titles: Vec<&str> = Page::ALL.iter().map(|p| p.label()).collect();
        let tabs_width: u16 =
            Page::ALL.iter().map(|p| p.label().len() as u16 + 2).sum::<u16>() + (Page::ALL.len() as u16 - 1) * 3;
        let [_, center_area, _] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Length(tabs_width), Constraint::Fill(1)])
                .areas(tabs_area);
        let tabs_widget = Tabs::new(tab_titles)
            .select(self.page.index())
            .style(Style::default().fg(Color::DarkGray))
            .highlight_style(Style::default().fg(COLOR).bold())
            .divider(" | ");
        frame.render_widget(tabs_widget, center_area);

        let indices = self.filtered_indices();

        let labels: Vec<String> = indices
            .iter()
            .map(|&i| {
                let todo = &self.items[i];
                let check = if todo.done { "[✓]" } else { "[ ]" };
                let repeat_icon = if todo.repeat.is_some() {
                    &format!("{} ", Repeat::icon())
                } else {
                    ""
                };
                let mut label = format!(" {} {}{}", check, repeat_icon, todo.text);
                if let Some(date) = todo.due_date {
                    label.push_str(&format!("  [{}]", date));
                }
                label
            })
            .collect();

        let max_width = labels.iter().map(|l| l.len() as u16).max().unwrap_or(0) + 2;
        let list_width = max_width.min(list_area.width);
        let h_offset = list_area.width.saturating_sub(list_width) / 2;

        let top_padding = 1;
        let centered_list_area = Rect {
            x: list_area.x + h_offset,
            y: list_area.y + top_padding,
            width: list_width,
            height: list_area.height.saturating_sub(top_padding),
        };

        let items: Vec<ListItem> = if matches!(self.page, Page::History) {
            labels
                .into_iter()
                .map(|label| {
                    ListItem::new(label).style(Style::default().fg(Color::DarkGray).add_modifier(Modifier::CROSSED_OUT))
                })
                .collect()
        } else {
            labels
                .into_iter()
                .zip(indices.iter())
                .map(|(label, &i)| {
                    let todo = &self.items[i];
                    let style = if todo.done {
                        Style::default().fg(Color::DarkGray).add_modifier(Modifier::CROSSED_OUT)
                    } else {
                        Style::default().fg(Color::White)
                    };
                    ListItem::new(label).style(style)
                })
                .collect()
        };

        let list = List::new(items)
            .highlight_style(Style::default().fg(self.color()).bold())
            .highlight_symbol(">");

        frame.render_stateful_widget(list, centered_list_area, &mut self.list_state.borrow_mut());

        let hint = match self.ui_mode {
            UiMode::Normal => match self.page {
                Page::History => "[[/]]Page  [j/k]Navigate",
                _ => "[[/]]Page  [j/k]Navigate  [Space]Toggle  [a]Add  [e]Edit  [d]Delete",
            },
            UiMode::Adding | UiMode::Editing => "[Enter]Confirm  [Esc]Cancel  [Backspace]Delete char",
        };

        frame.render_widget(Paragraph::new(hint).centered().fg(Color::DarkGray), hint_area);

        if let Some(input_rect) = input_area {
            if let Some(input_area_widget) = &self.input_widget {
                frame.render_widget(input_area_widget, input_rect);
                input_area_widget.render_calendar(frame, area);
            }
        }
    }
}
