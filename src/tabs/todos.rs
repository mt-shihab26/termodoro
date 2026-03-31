use std::cell::RefCell;
use std::io::Result;

use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::widgets::{Block, List, ListItem, ListState, Paragraph, Tabs, Widget};
use sea_orm::DatabaseConnection;

use crate::kinds::ui_mode::UiMode;
use crate::kinds::{page::Page, repeat::Repeat};
use crate::models::todo::Todo;
use crate::widgets::input::{InputAction, InputWidget};

use super::Tab;

pub const COLOR: Color = Color::Green;

pub struct Todos {
    db: DatabaseConnection,
    page: Page,
    ui_mode: UiMode,
    selected: usize,
    list_state: RefCell<ListState>,
    cache: RefCell<Option<Vec<Todo>>>,
    input_widget: Option<InputWidget>,
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
                KeyCode::Char('1') => self.set_page(Page::Due),
                KeyCode::Char('2') => self.set_page(Page::Today),
                KeyCode::Char('3') => self.set_page(Page::Index),
                KeyCode::Char('4') => self.set_page(Page::History),
                KeyCode::Char(']') => self.set_page(self.page.next()),
                KeyCode::Char('[') => self.set_page(self.page.prev()),
                KeyCode::Char('j') | KeyCode::Down => self.move_selection(1),
                KeyCode::Char('k') | KeyCode::Up => self.move_selection(-1),
                KeyCode::Char(' ') | KeyCode::Enter => {
                    if let Some(mut todo) = self.selected_item() {
                        todo.toggle(&self.db);
                        self.refresh();
                    }
                }
                KeyCode::Char('d') => {
                    if !matches!(self.page, Page::History) {
                        if let Some(todo) = self.selected_item() {
                            if !todo.done {
                                todo.delete(&self.db);
                                self.refresh();
                            }
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
                        if let Some(todo) = self.selected_item() {
                            self.ui_mode = UiMode::Editing;
                            self.input_widget =
                                Some(InputWidget::new(Some(&todo.text), todo.due_date, todo.repeat.as_ref()));
                        }
                    }
                }
                _ => {}
            },
            UiMode::Adding => {
                if let Some(input_widget) = &mut self.input_widget {
                    match input_widget.handle(key) {
                        InputAction::Confirm { text, date, repeat } => self.confirm_add(text, date, repeat),
                        InputAction::Escape => self.cancel_input(),
                        InputAction::None => {}
                    }
                }
            }
            UiMode::Editing => {
                if let Some(input_widget) = &mut self.input_widget {
                    match input_widget.handle(key) {
                        InputAction::Confirm { text, date, repeat } => self.confirm_edit(text, date, repeat),
                        InputAction::Escape => self.cancel_input(),
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

        let items = self.current_items();

        let labels: Vec<String> = items
            .iter()
            .map(|todo| {
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

        let horizontal_padding = 2;
        let top_padding = 1;
        let padded_list_area = Rect {
            x: list_area.x + horizontal_padding,
            y: list_area.y + top_padding,
            width: list_area.width.saturating_sub(horizontal_padding * 2),
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
                .zip(items.iter())
                .map(|(label, todo)| {
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

        frame.render_stateful_widget(list, padded_list_area, &mut self.list_state.borrow_mut());

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

impl Todos {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db,
            page: Page::Today,
            ui_mode: UiMode::Normal,
            selected: 0,
            list_state: RefCell::new(ListState::default()),
            cache: RefCell::new(None),
            input_widget: None,
        }
    }

    fn current_items(&self) -> Vec<Todo> {
        {
            let mut cache = self.cache.borrow_mut();
            if cache.is_none() {
                *cache = Some(Todo::list(&self.db, self.page));
            }
        }
        self.cache.borrow().as_deref().unwrap_or(&[]).to_vec()
    }

    fn invalidate_cache(&self) {
        *self.cache.borrow_mut() = None;
    }

    fn refresh(&mut self) {
        self.invalidate_cache();
        self.clamp_selected();
    }

    fn selected_item(&self) -> Option<Todo> {
        self.current_items().get(self.selected).cloned()
    }

    fn clamp_selected(&mut self) {
        let len = self.current_items().len();
        if len == 0 {
            self.selected = 0;
        } else {
            self.selected = self.selected.min(len - 1);
        }
    }

    fn sync_list_state(&self) {
        let len = self.current_items().len();
        let selected = if len == 0 {
            None
        } else {
            Some(self.selected.min(len - 1))
        };
        self.list_state.borrow_mut().select(selected);
    }

    fn set_page(&mut self, page: Page) {
        self.page = page;
        self.selected = 0;
        self.invalidate_cache();
    }

    fn move_selection(&mut self, delta: isize) {
        let len = self.current_items().len();
        if len == 0 {
            self.selected = 0;
            return;
        }

        self.selected = self.selected.saturating_add_signed(delta).min(len - 1);
    }

    fn cancel_input(&mut self) {
        self.input_widget = None;
        self.ui_mode = UiMode::Normal;
    }

    fn confirm_add(&mut self, text: String, date: Option<time::Date>, repeat: Option<Repeat>) {
        let mut todo = Todo::new(text, date, repeat);
        if todo.save(&self.db) {
            self.refresh();
        }
        self.cancel_input();
    }

    fn confirm_edit(&mut self, text: String, date: Option<time::Date>, repeat: Option<Repeat>) {
        if let Some(mut todo) = self.selected_item() {
            todo.text = text;
            todo.due_date = date;
            todo.repeat = repeat;
            todo.update(&self.db);
            self.refresh();
        }
        self.cancel_input();
    }
}
