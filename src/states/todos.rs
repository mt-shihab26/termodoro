use std::{
    cell::{Cell, Ref, RefCell, RefMut},
    sync::{Arc, Mutex},
};

use ratatui::{layout::Rect, widgets::ListState};
use sea_orm::DatabaseConnection;
use time::Date;

use crate::{
    caches::{timer::TimerCache, todos::TodosCache},
    kinds::{page::Page, repeat::Repeat},
    models::{
        session::{Session, Stat},
        todo::Todo,
    },
};

/// Runtime state for the todos tab, managing pagination, selection, and data caches.
pub struct TodosState {
    /// Database connection used for all todo and session queries.
    db: DatabaseConnection,
    /// Whether a `g` keypress is pending (waiting for a second `g` to jump to top).
    pending_g: bool,
    /// Index of the currently selected row within the visible page.
    selected: usize,
    /// Global offset into the full todo list for the current page view.
    offset: usize,
    /// Number of items that fit in the visible list area; updated on each resize.
    page_size: Cell<usize>,
    /// ratatui list state kept in sync for scroll tracking.
    list_state: RefCell<ListState>,
    /// Cache for the todos list, keyed by page and pagination params.
    todos_cache: TodosCache,
    /// Shared timer cache, invalidated when todos change.
    timer_cache: Arc<Mutex<TimerCache>>,
}

impl TodosState {
    /// Creates a new `TodosState` connected to the given database and timer cache.
    pub fn new(db: DatabaseConnection, timer_cache: Arc<Mutex<TimerCache>>) -> Self {
        Self {
            todos_cache: TodosCache::new(db.clone()),
            db,
            pending_g: false,
            selected: 0,
            offset: 0,
            page_size: Cell::new(1),
            list_state: RefCell::new(ListState::default()),
            timer_cache,
        }
    }

    /// Returns the index of the currently selected row.
    pub fn selected(&self) -> usize {
        self.selected
    }

    /// Returns the current pagination offset.
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Returns a mutable borrow of the ratatui list state.
    pub fn list_state_mut(&self) -> RefMut<'_, ListState> {
        self.list_state.borrow_mut()
    }

    /// Consumes and returns the pending-`g` flag, resetting it to `false`.
    pub fn begin_input(&mut self) -> bool {
        let pending_g = self.pending_g;
        self.pending_g = false;
        pending_g
    }

    /// Returns the 1-based index of the first visible item, or 0 if the list is empty.
    pub fn from(&self, total: usize) -> usize {
        if total == 0 { 0 } else { self.offset + 1 }
    }

    /// Returns the 1-based index of the last visible item.
    pub fn to(&self, loaded_len: usize) -> usize {
        self.offset + loaded_len
    }

    /// Returns the current 1-based page number.
    pub fn page(&self) -> usize {
        (self.offset / self.page_size()) + 1
    }

    /// Returns `true` if there are hidden items above the visible window.
    pub fn show_more_above(&self) -> bool {
        self.offset > 0
    }

    /// Returns `true` if there are hidden items below the visible window.
    pub fn show_more_below(&self, loaded_len: usize) -> bool {
        loaded_len == self.page_size()
    }

    /// Returns session stats for each visible todo on the given page.
    pub fn stats(&self, page: Page) -> Vec<Option<Stat>> {
        self.todos_cache
            .get_items(page, self.offset, self.page_size())
            .iter()
            .map(|t| t.id.map(|id| Session::stat(&self.db, id)))
            .collect()
    }

    /// Returns a borrowed slice of the visible todos for the given page.
    pub fn items(&self, page: Page) -> Ref<'_, [Todo]> {
        self.todos_cache.get_items(page, self.offset, self.page_size())
    }

    /// Returns the total number of todos for the given page.
    pub fn count(&self, page: Page) -> usize {
        self.todos_cache.get_count(page)
    }

    /// Returns a borrowed reference to the currently selected todo, if any.
    pub fn selected_item(&self, page: Page) -> Option<Ref<'_, Todo>> {
        self.todos_cache
            .get_item_at(page, self.offset, self.page_size(), self.selected)
    }

    /// Updates the visible capacity based on the rendered list area and invalidates caches on change.
    pub fn set_visible_capacity(&self, list_area: Rect) {
        let top_padding = 1usize;
        let capacity = list_area.height.saturating_sub(top_padding as u16) as usize;
        let capacity = capacity.max(1);

        if self.page_size.get() != capacity {
            self.page_size.set(capacity);
            self.clear_caches();
        }
    }

    /// Returns the current page size, ensuring it is at least 1.
    pub fn page_size(&self) -> usize {
        self.page_size.get().max(1)
    }

    /// Invalidates all todo caches.
    fn clear_caches(&self) {
        self.todos_cache.invalidate_all();
    }

    /// Invalidates the timer cache's todo list so the picker reflects recent changes.
    fn invalidate_timer_todos(&self) {
        if let Ok(mut c) = self.timer_cache.lock() {
            c.invalidate_todos();
        }
    }

    /// Invalidates only the items cache, keeping counts intact.
    fn invalidate_items(&self) {
        self.todos_cache.invalidate_items();
    }

    /// Returns `true` if the selected todo on the given page can be deleted.
    pub fn can_delete(&self, page: Page, items: &[Todo]) -> bool {
        !matches!(page, Page::History) && items.get(self.selected).is_some_and(|todo| todo.done_at.is_none())
    }

    /// Clamps the selection to valid bounds, scrolling back a page if the current page is empty.
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

    /// Syncs the ratatui list state selection to the current `selected` index.
    pub fn sync_list_state(&self, len: usize) {
        let selected = if len == 0 {
            None
        } else {
            Some(self.selected.min(len - 1))
        };
        self.list_state.borrow_mut().select(selected);
    }

    /// Resets pagination and selection for the given page, jumping to today's items where applicable.
    pub fn reset_page(&mut self, page: Page) {
        self.pending_g = false;
        self.offset = 0;
        self.selected = 0;
        self.clear_caches();

        let done = match page {
            Page::Index => Some(false),
            Page::History => Some(true),
            _ => None,
        };
        if let Some(done) = done {
            let before = Todo::count_before_today(&self.db, done);
            let page_size = self.page_size();
            self.offset = (before / page_size) * page_size;
            self.selected = before - self.offset;
            self.invalidate_items();
        }

        self.clamp_selected(page);
    }

    /// Jumps to the top of the list on a double `g` press.
    pub fn go_to_start(&mut self, pending_g: bool) {
        if pending_g {
            self.offset = 0;
            self.selected = 0;
            self.invalidate_items();
        }
        self.pending_g = !pending_g;
    }

    /// Jumps to the last item in the list.
    pub fn go_to_end(&mut self, page: Page) {
        let total = Todo::count(&self.db, page);
        if total == 0 {
            return;
        }
        let page_size = self.page_size();
        self.offset = total.saturating_sub(page_size);
        self.selected = (total - 1) - self.offset;
        self.invalidate_items();
    }

    /// Refreshes caches and clamps the selection after a data mutation.
    pub fn refresh(&mut self, page: Page) {
        self.clear_caches();
        self.invalidate_timer_todos();
        self.clamp_selected(page);
    }

    /// Adds a new todo and refreshes the list.
    pub fn add(&mut self, page: Page, text: String, due_date: Option<Date>, repeat: Option<Repeat>) {
        let mut todo = Todo::new(text, due_date, repeat, None);
        if todo.save(&self.db) {
            self.refresh(page);
        }
    }

    /// Updates the selected todo's text, due date, and repeat rule, then refreshes.
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

    /// Moves the selection by `delta` rows, scrolling the page when reaching the edge.
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

    /// Toggles the done state of the selected todo and refreshes the list.
    pub fn toggle_selected(&mut self, page: Page) {
        if let Some(mut todo) = self.selected_item(page).map(|todo| todo.clone()) {
            todo.toggle(&self.db);
            self.refresh(page);
        }
    }

    /// Deletes the selected todo if it is not done, then refreshes the list.
    pub fn delete_selected(&mut self, page: Page) {
        let deleted = {
            let todo = match self.selected_item(page) {
                Some(todo) => todo,
                None => return (),
            };
            if todo.done_at.is_some() {
                return ();
            }
            todo.delete(&self.db)
        };

        if deleted {
            self.refresh(page);
        }
    }

    /// Returns the text, due date, and repeat rule of the selected todo for editing.
    pub fn edit_values(&self, page: Page) -> Option<(String, Option<time::Date>, Option<Repeat>)> {
        self.selected_item(page)
            .map(|todo| (todo.text.clone(), todo.due_date, todo.repeat.clone()))
    }
}
