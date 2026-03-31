use std::cell::{Cell, Ref, RefCell};
use std::io::Result;

use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Block, ListState, Widget};
use sea_orm::DatabaseConnection;

use crate::kinds::direction::Direction;
use crate::kinds::mode::Mode;
use crate::kinds::{page::Page, repeat::Repeat};
use crate::models::todo::Todo;
use crate::widgets::todos::hint::HintWidget;
use crate::widgets::todos::input::{InputAction, InputWidget};
use crate::widgets::todos::list::ListWidget;
use crate::widgets::todos::status::StatusWidget;
use crate::widgets::todos::tabs::TabsWidget;

use super::Tab;

pub const COLOR: Color = Color::Green;

pub struct Todos {
    db: DatabaseConnection,
    page: Page,
    mode: Mode,
    pending_g: bool,
    direction: Option<Direction>,
    selected: usize,
    offset: usize,
    page_size: Cell<usize>,
    list_state: RefCell<ListState>,
    items_cache: RefCell<Option<Vec<Todo>>>,
    count_cache: RefCell<Option<usize>>,
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
        let pending_g = self.pending_g;
        self.pending_g = false;
        self.direction = None;

        match self.mode {
            Mode::Normal => match key.code {
                KeyCode::Char('1') => self.set_page(Page::Due),
                KeyCode::Char('2') => self.set_page(Page::Today),
                KeyCode::Char('3') => self.set_page(Page::Index),
                KeyCode::Char('4') => self.set_page(Page::History),
                KeyCode::Char(']') => self.set_page(self.page.next()),
                KeyCode::Char('[') => self.set_page(self.page.prev()),
                KeyCode::Char('j') | KeyCode::Down => self.move_selection(1),
                KeyCode::Char('k') | KeyCode::Up => self.move_selection(-1),
                KeyCode::Char('g') => self.go_to_start(pending_g),
                KeyCode::Char('G') => self.go_to_end(),
                KeyCode::Char(' ') | KeyCode::Enter => {
                    if let Some(mut todo) = self.selected_item().map(|todo| todo.clone()) {
                        todo.toggle(&self.db);
                        self.refresh();
                    }
                }
                KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    if self.can_delete_selected() {
                        if let Some(should_delete) = self.selected_item().map(|todo| {
                            if todo.done {
                                false
                            } else {
                                todo.delete(&self.db)
                            }
                        }) {
                            if should_delete {
                                self.refresh();
                            }
                        }
                    }
                }
                KeyCode::Char('a') => {
                    if !matches!(self.page, Page::History) {
                        self.mode = Mode::Adding;
                        self.input_widget = Some(InputWidget::new(None, None, None));
                    }
                }
                KeyCode::Char('e') => {
                    if !matches!(self.page, Page::History) {
                        if let Some((text, due_date, repeat)) = self
                            .selected_item()
                            .map(|todo| (todo.text.clone(), todo.due_date, todo.repeat.clone()))
                        {
                            self.mode = Mode::Editing;
                            self.input_widget =
                                Some(InputWidget::new(Some(&text), due_date, repeat.as_ref()));
                        }
                    }
                }
                _ => {}
            },
            Mode::Adding => {
                if let Some(input_widget) = &mut self.input_widget {
                    match input_widget.handle(key) {
                        InputAction::Confirm { text, date, repeat } => {
                            self.confirm_add(text, date, repeat)
                        }
                        InputAction::Escape => self.cancel_input(),
                        InputAction::None => {}
                    }
                }
            }
            Mode::Editing => {
                if let Some(input_widget) = &mut self.input_widget {
                    match input_widget.handle(key) {
                        InputAction::Confirm { text, date, repeat } => {
                            self.confirm_edit(text, date, repeat)
                        }
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

        let (tabs_area, list_area, hint_area, input_area) = match self.mode {
            Mode::Normal => {
                let [tabs, list, hint] = Layout::vertical([
                    Constraint::Length(1),
                    Constraint::Fill(1),
                    Constraint::Length(1),
                ])
                .areas(area);
                (tabs, list, hint, None)
            }
            Mode::Adding | Mode::Editing => {
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
        let items = self.items();
        let total = self.count();
        let from = if total == 0 { 0 } else { self.offset + 1 };
        let to = self.offset + items.len();
        let page = (self.offset / self.page_size.get().max(1)) + 1;

        frame.render_widget(
            StatusWidget::new(
                total,
                from,
                to,
                page,
                items.get(self.selected).and_then(|todo| todo.id),
            ),
            area,
        );

        frame.render_widget(
            TabsWidget {
                page: self.page,
                color: self.color(),
            },
            tabs_area,
        );

        ListWidget {
            items: &items,
            offset: self.offset,
            page: self.page,
            selected: self.selected,
            color: self.color(),
            show_more_above: self.offset > 0,
            show_more_below: items.len() == self.page_size.get(),
        }
        .render(frame, list_area, &mut self.list_state.borrow_mut());

        frame.render_widget(
            HintWidget {
                page: self.page,
                ui_mode: self.mode,
                can_delete: self.can_delete_in_items(&items),
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

    fn should_tick(&self) -> bool {
        self.direction.is_some()
    }

    fn next_tick(&mut self) -> Result<()> {
        if !matches!(self.mode, Mode::Normal) {
            return Ok(());
        }

        if let Some(direction) = &self.direction {
            let position = (self.offset, self.selected);

            match direction {
                Direction::Start => self.move_selection(-1),
                Direction::End => self.move_selection(1),
            }

            if (self.offset, self.selected) == position {
                self.direction = None;
            }

            self.sync_list_state();
        }

        Ok(())
    }
}

impl Todos {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db,
            page: Page::Today,
            mode: Mode::Normal,
            pending_g: false,
            direction: None,
            selected: 0,
            offset: 0,
            page_size: Cell::new(1),
            list_state: RefCell::new(ListState::default()),
            items_cache: RefCell::new(None),
            count_cache: RefCell::new(None),
            input_widget: None,
        }
    }

    fn items(&self) -> Ref<'_, [Todo]> {
        let mut items_cache = self.items_cache.borrow_mut();
        if items_cache.is_none() {
            *items_cache = Some(Todo::list(
                &self.db,
                self.page,
                self.offset,
                self.page_size.get(),
            ));
        }
        Ref::map(self.items_cache.borrow(), |cache| {
            cache.as_deref().unwrap_or(&[])
        })
    }

    fn count(&self) -> usize {
        let mut cache = self.count_cache.borrow_mut();
        if cache.is_none() {
            *cache = Some(Todo::count(&self.db, self.page));
        }
        cache.unwrap_or(0)
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
        *self.items_cache.borrow_mut() = None;
    }

    fn invalidate_total_count_cache(&self) {
        *self.count_cache.borrow_mut() = None;
    }

    fn refresh(&mut self) {
        self.invalidate_cache();
        self.invalidate_total_count_cache();
        self.clamp_selected();
    }

    fn selected_item(&self) -> Option<Ref<'_, Todo>> {
        self.items();

        let cache = self.items_cache.borrow();
        if cache
            .as_ref()
            .and_then(|items| items.get(self.selected))
            .is_none()
        {
            return None;
        }

        Some(Ref::map(cache, |cache| {
            &cache.as_ref().unwrap()[self.selected]
        }))
    }

    fn can_delete_selected(&self) -> bool {
        let items = self.items();
        self.can_delete_in_items(&items)
    }

    fn can_delete_in_items(&self, items: &[Todo]) -> bool {
        !matches!(self.page, Page::History)
            && items.get(self.selected).is_some_and(|todo| !todo.done)
    }

    fn clamp_selected(&mut self) {
        let mut len = self.items().len();
        if len == 0 && self.offset > 0 {
            self.offset = self.offset.saturating_sub(self.page_size.get().max(1));
            self.invalidate_cache();
            len = self.items().len();
        }

        if len == 0 {
            self.selected = 0;
        } else {
            self.selected = self.selected.min(len - 1);
        }
    }

    fn sync_list_state(&self) {
        let len = self.items().len();
        let selected = if len == 0 {
            None
        } else {
            Some(self.selected.min(len - 1))
        };
        self.list_state.borrow_mut().select(selected);
    }

    fn set_page(&mut self, page: Page) {
        self.page = page;
        self.pending_g = false;
        self.direction = None;
        self.offset = 0;
        self.selected = 0;
        self.invalidate_cache();
        self.invalidate_total_count_cache();
    }

    fn go_to_start(&mut self, pending_g: bool) {
        if pending_g {
            self.direction = Some(Direction::Start);
        } else {
            self.direction = None;
        }
        self.pending_g = !pending_g;
    }

    fn go_to_end(&mut self) {
        self.direction = Some(Direction::End);
    }

    fn move_selection(&mut self, delta: isize) {
        if delta > 0 {
            for _ in 0..delta as usize {
                let len = self.items().len();
                if len == 0 {
                    self.selected = 0;
                    break;
                }

                if self.selected + 1 < len {
                    self.selected += 1;
                } else if len == self.page_size.get().max(1) {
                    self.offset += 1;
                    self.invalidate_cache();
                } else {
                    break;
                }
            }
        } else if delta < 0 {
            for _ in 0..delta.unsigned_abs() {
                let len = self.items().len();
                if len == 0 {
                    self.selected = 0;
                    break;
                }

                if self.selected > 0 {
                    self.selected -= 1;
                } else if self.offset > 0 {
                    self.offset -= 1;
                    self.invalidate_cache();
                } else {
                    break;
                }
            }
        }
    }

    fn cancel_input(&mut self) {
        self.input_widget = None;
        self.mode = Mode::Normal;
    }

    fn confirm_add(&mut self, text: String, date: Option<time::Date>, repeat: Option<Repeat>) {
        let mut todo = Todo::new(text, date, repeat);
        if todo.save(&self.db) {
            self.refresh();
        }
        self.cancel_input();
    }

    fn confirm_edit(&mut self, text: String, date: Option<time::Date>, repeat: Option<Repeat>) {
        if let Some(mut todo) = self.selected_item().map(|todo| todo.clone()) {
            todo.text = text;
            todo.due_date = date;
            todo.repeat = repeat;
            todo.update(&self.db);
            self.refresh();
        }
        self.cancel_input();
    }
}
