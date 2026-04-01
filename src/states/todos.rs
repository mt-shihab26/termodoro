use std::cell::{Cell, Ref, RefCell, RefMut};
use std::sync::{Arc, Mutex};

use ratatui::{layout::Rect, widgets::ListState};
use sea_orm::DatabaseConnection;
use time::Date;

use crate::kinds::{direction::Direction, page::Page, repeat::Repeat};
use crate::models::{session::Session, todo::Todo};
use crate::states::timer_cache::TimerCache;

pub struct TodosState {
    db: DatabaseConnection,
    pending_g: bool,
    direction: Option<Direction>,
    selected: usize,
    offset: usize,
    page_size: Cell<usize>,
    list_state: RefCell<ListState>,
    items: RefCell<Option<Vec<Todo>>>,
    count: RefCell<Option<usize>>,
    timer_cache: Arc<Mutex<TimerCache>>,
}

impl TodosState {
    pub fn new(db: DatabaseConnection, timer_cache: Arc<Mutex<TimerCache>>) -> Self {
        Self {
            db,
            pending_g: false,
            direction: None,
            selected: 0,
            offset: 0,
            page_size: Cell::new(1),
            list_state: RefCell::new(ListState::default()),
            items: RefCell::new(None),
            count: RefCell::new(None),
            timer_cache,
        }
    }

    pub fn begin_input(&mut self) -> bool {
        let pending_g = self.pending_g;
        self.pending_g = false;
        self.direction = None;
        pending_g
    }

    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn list_state_mut(&self) -> RefMut<'_, ListState> {
        self.list_state.borrow_mut()
    }

    pub fn from(&self, total: usize) -> usize {
        if total == 0 { 0 } else { self.offset + 1 }
    }

    pub fn to(&self, loaded_len: usize) -> usize {
        self.offset + loaded_len
    }

    pub fn page(&self) -> usize {
        (self.offset / self.page_size()) + 1
    }

    pub fn show_more_above(&self) -> bool {
        self.offset > 0
    }

    pub fn show_more_below(&self, loaded_len: usize) -> bool {
        loaded_len == self.page_size()
    }

    pub fn should_tick(&self) -> bool {
        self.direction.is_some()
    }

    pub fn is_animating(&self) -> bool {
        self.direction.is_some()
    }

    pub fn stop_animation(&mut self) {
        self.direction = None;
    }

    pub fn stats(&self, page: Page) -> Vec<Option<(u32, u32)>> {
        // Only call borrow_mut when the cache is empty. If the caller already
        // holds a Ref from items(), the cache is populated and we skip straight
        // to the immutable borrow below — two simultaneous Refs are allowed.
        if self.items.borrow().is_none() {
            let mut guard = self.items.borrow_mut();
            if guard.is_none() {
                *guard = Some(Todo::list(&self.db, page, self.offset, self.page_size()));
            }
        }

        self.items
            .borrow()
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .map(|t| t.id.map(|id| Session::stats_for_todo(&self.db, id)))
            .collect()
    }

    pub fn items(&self, page: Page) -> Ref<'_, [Todo]> {
        let mut items = self.items.borrow_mut();
        if items.is_none() {
            *items = Some(Todo::list(&self.db, page, self.offset, self.page_size()));
        }
        drop(items);

        Ref::map(self.items.borrow(), |cache| cache.as_deref().unwrap_or(&[]))
    }

    pub fn count(&self, page: Page) -> usize {
        let mut count = self.count.borrow_mut();
        if count.is_none() {
            *count = Some(Todo::count(&self.db, page));
        }
        count.unwrap_or(0)
    }

    pub fn selected_item(&self, page: Page) -> Option<Ref<'_, Todo>> {
        self.items(page);

        let cache = self.items.borrow();
        if cache.as_ref().and_then(|items| items.get(self.selected)).is_none() {
            return None;
        }

        Some(Ref::map(cache, |cache| &cache.as_ref().unwrap()[self.selected]))
    }

    pub fn set_visible_capacity(&self, list_area: Rect) {
        let top_padding = 1usize;
        let capacity = list_area.height.saturating_sub(top_padding as u16) as usize;
        let capacity = capacity.max(1);

        if self.page_size.get() != capacity {
            self.page_size.set(capacity);
            self.clear_caches();
        }
    }

    pub fn page_size(&self) -> usize {
        self.page_size.get().max(1)
    }

    fn clear_caches(&self) {
        self.invalidate_items();
        self.invalidate_count();
    }

    fn invalidate_timer_todos(&self) {
        if let Ok(mut c) = self.timer_cache.lock() {
            c.invalidate_todos();
        }
    }

    fn invalidate_items(&self) {
        *self.items.borrow_mut() = None;
    }

    fn invalidate_count(&self) {
        *self.count.borrow_mut() = None;
    }

    pub fn can_delete(&self, page: Page, items: &[Todo]) -> bool {
        !matches!(page, Page::History) && items.get(self.selected).is_some_and(|todo| !todo.done)
    }

    pub fn clamp_selected(&mut self, page: Page) {
        let mut len = self.items(page).len();
        if len == 0 && self.offset > 0 {
            self.offset = self.offset.saturating_sub(self.page_size());
            self.invalidate_items();
            len = self.items(page).len();
        }

        if len == 0 {
            self.selected = 0;
        } else {
            self.selected = self.selected.min(len - 1);
        }
    }

    pub fn sync_list_state(&self, len: usize) {
        let selected = if len == 0 {
            None
        } else {
            Some(self.selected.min(len - 1))
        };
        self.list_state.borrow_mut().select(selected);
    }

    pub fn reset_page(&mut self, page: Page) {
        self.pending_g = false;
        self.direction = None;
        self.offset = 0;
        self.selected = 0;
        self.clear_caches();
        self.clamp_selected(page);
    }

    pub fn go_to_start(&mut self, pending_g: bool) {
        if pending_g {
            self.direction = Some(Direction::Start);
        } else {
            self.direction = None;
        }
        self.pending_g = !pending_g;
    }

    pub fn go_to_end(&mut self) {
        self.direction = Some(Direction::End);
    }

    pub fn refresh(&mut self, page: Page) {
        self.clear_caches();
        self.invalidate_timer_todos();
        self.clamp_selected(page);
    }

    pub fn add(&mut self, page: Page, text: String, due_date: Option<Date>, repeat: Option<Repeat>) {
        let mut todo = Todo::new(text, due_date, repeat);
        if todo.save(&self.db) {
            self.refresh(page);
        }
    }

    pub fn update(&mut self, page: Page, text: String, due_date: Option<Date>, repeat: Option<Repeat>) {
        if let Some(mut todo) = self.selected_item(page).map(|todo| todo.clone()) {
            todo.text = text;
            todo.due_date = due_date;
            todo.repeat = repeat;
            if todo.update(&self.db) {
                self.refresh(page);
            }
        }
    }

    pub fn move_selection(&mut self, page: Page, delta: isize) {
        if delta > 0 {
            for _ in 0..delta as usize {
                let len = self.items(page).len();
                if len == 0 {
                    self.selected = 0;
                    break;
                }

                if self.selected + 1 < len {
                    self.selected += 1;
                } else if len == self.page_size() {
                    self.offset += 1;
                    self.invalidate_items();
                } else {
                    break;
                }
            }
        } else if delta < 0 {
            for _ in 0..delta.unsigned_abs() {
                let len = self.items(page).len();
                if len == 0 {
                    self.selected = 0;
                    break;
                }

                if self.selected > 0 {
                    self.selected -= 1;
                } else if self.offset > 0 {
                    self.offset -= 1;
                    self.invalidate_items();
                } else {
                    break;
                }
            }
        }
    }

    pub fn step_animation(&mut self, page: Page) -> bool {
        let position = (self.offset, self.selected);

        match self.direction {
            Some(Direction::Start) => self.move_selection(page, -1),
            Some(Direction::End) => self.move_selection(page, 1),
            None => {}
        }

        (self.offset, self.selected) != position
    }

    pub fn toggle_selected(&mut self, page: Page) {
        if let Some(mut todo) = self.selected_item(page).map(|todo| todo.clone()) {
            todo.toggle(&self.db);
            self.refresh(page);
        }
    }

    pub fn delete_selected(&mut self, page: Page) {
        let deleted = {
            let todo = match self.selected_item(page) {
                Some(todo) => todo,
                None => return (),
            };
            if todo.done {
                return ();
            }
            todo.delete(&self.db)
        };

        if deleted {
            self.refresh(page);
        }
    }

    pub fn edit_values(&self, page: Page) -> Option<(String, Option<time::Date>, Option<Repeat>)> {
        self.selected_item(page)
            .map(|todo| (todo.text.clone(), todo.due_date, todo.repeat.clone()))
    }
}
