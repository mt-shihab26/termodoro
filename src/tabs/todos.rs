use std::cell::{Cell, RefCell};
use std::io::Result;

use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Block, ListState, Widget};
use sea_orm::DatabaseConnection;

use crate::kinds::ui_mode::UiMode;
use crate::kinds::{page::Page, repeat::Repeat};
use crate::models::todo::Todo;
use crate::widgets::input::{InputAction, InputWidget};
use crate::widgets::todos_cache_status::TodosCacheStatusWidget;
use crate::widgets::todos_dated_list::TodosDatedListWidget;
use crate::widgets::todos_hint::TodosHintWidget;
use crate::widgets::todos_index::TodosIndexWidget;
use crate::widgets::todos_tabs::TodosTabsWidget;

use super::Tab;

pub const COLOR: Color = Color::Green;

pub struct Todos {
    db: DatabaseConnection,
    page: Page,
    ui_mode: UiMode,
    selected: usize,
    offset: usize,
    page_size: Cell<usize>,
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
                KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
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

        frame.render_widget(TodosCacheStatusWidget::new(self.current_items().len()), area);

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

        self.set_visible_capacity(list_area);

        frame.render_widget(
            TodosTabsWidget {
                page: self.page,
                color: self.color(),
            },
            tabs_area,
        );

        let items = self.current_items();
        match self.page {
            Page::Index => TodosIndexWidget {
                items: &items,
                selected: self.selected,
                color: self.color(),
            }
            .render(frame, list_area),
            Page::Due | Page::Today | Page::History => TodosDatedListWidget {
                items: &items,
                dimmed: matches!(self.page, Page::History),
                color: self.color(),
            }
            .render(frame, list_area, &mut self.list_state.borrow_mut()),
        }

        frame.render_widget(
            TodosHintWidget {
                page: self.page,
                ui_mode: self.ui_mode,
            },
            hint_area,
        );

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
            offset: 0,
            page_size: Cell::new(1),
            list_state: RefCell::new(ListState::default()),
            cache: RefCell::new(None),
            input_widget: None,
        }
    }

    fn current_items(&self) -> Vec<Todo> {
        {
            let mut cache = self.cache.borrow_mut();
            if cache.is_none() {
                *cache = Some(Todo::list(&self.db, self.page, self.offset, self.page_size.get()));
            }
        }
        self.cache.borrow().as_deref().unwrap_or(&[]).to_vec()
    }

    fn set_visible_capacity(&self, list_area: Rect) {
        let top_padding = 1usize;
        let capacity = list_area.height.saturating_sub(top_padding as u16) as usize;
        let capacity = capacity.max(1);

        if self.page_size.get() != capacity {
            self.page_size.set(capacity);
            self.invalidate_cache();
        }
    }

    fn invalidate_cache(&self) {
        *self.cache.borrow_mut() = None;
    }

    // fn cache_status(&self) -> String {
    //     format!(" loaded {} ", self.current_items().len())
    // }

    fn refresh(&mut self) {
        self.invalidate_cache();
        self.clamp_selected();
    }

    fn selected_item(&self) -> Option<Todo> {
        self.current_items().get(self.selected).cloned()
    }

    fn clamp_selected(&mut self) {
        let mut len = self.current_items().len();
        if len == 0 && self.offset > 0 {
            self.offset = self.offset.saturating_sub(self.page_size.get().max(1));
            self.invalidate_cache();
            len = self.current_items().len();
        }

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
        self.offset = 0;
        self.selected = 0;
        self.invalidate_cache();
    }

    fn move_selection(&mut self, delta: isize) {
        let len = self.current_items().len();
        if len == 0 {
            self.selected = 0;
            return;
        }

        if delta > 0 {
            if self.selected + 1 < len {
                self.selected += 1;
            } else if len == self.page_size.get().max(1) {
                self.offset += 1;
                self.invalidate_cache();
            }
        } else if delta < 0 {
            if self.selected > 0 {
                self.selected -= 1;
            } else if self.offset > 0 {
                self.offset -= 1;
                self.invalidate_cache();
            }
        }
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
