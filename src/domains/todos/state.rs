use time::Date;

use crate::domains::todos::Repeat;

use super::{mode::Mode, todo::Todo};

pub struct TodosState {
    pub items: Vec<Todo>,
    pub selected: usize,
    pub mode: Mode,
    pub input: String,
    pub editing_idx: Option<usize>,
}

impl TodosState {
    pub fn new() -> Self {
        Self {
            items: Todo::fakes(),
            selected: 0,
            mode: Mode::Normal,
            input: String::new(),
            editing_idx: None,
        }
    }

    pub fn move_down(&mut self) {
        if !self.items.is_empty() {
            self.selected = (self.selected + 1).min(self.items.len() - 1);
        }
    }

    pub fn move_up(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    pub fn toggle_selected(&mut self) {
        if !self.items.is_empty() {
            self.items[self.selected].done = !self.items[self.selected].done;
        }
    }

    pub fn delete_selected(&mut self) {
        if !self.items.is_empty() {
            self.items.remove(self.selected);
            if !self.items.is_empty() {
                self.selected = self.selected.min(self.items.len() - 1);
            } else {
                self.selected = 0;
            }
        }
    }

    pub fn start_adding(&mut self) {
        self.mode = Mode::Adding;
        self.input.clear();
    }

    /// Advance to date selection if input is non-empty, otherwise cancel.
    pub fn confirm_add(&mut self) {
        if !self.input.trim().is_empty() {
            self.editing_idx = None;
            self.mode = Mode::SelectingDate;
        } else {
            self.mode = Mode::Normal;
            self.input.clear();
        }
    }

    pub fn cancel_add(&mut self) {
        self.mode = Mode::Normal;
        self.input.clear();
    }

    /// Open the calendar to edit the due date of the selected todo.
    pub fn start_edit_date(&mut self) {
        if self.items.is_empty() {
            return;
        }
        self.editing_idx = Some(self.selected);
        self.mode = Mode::SelectingDate;
    }

    /// Discard the in-progress add/edit and return to Normal.
    pub fn cancel_selecting_date(&mut self) {
        if self.editing_idx.is_none() {
            self.input.clear();
        }
        self.editing_idx = None;
        self.mode = Mode::Normal;
    }

    /// Commit the selected date and repeat to a new or existing todo.
    pub fn confirm_with(&mut self, date: Date, repeat: Repeat) {
        if let Some(idx) = self.editing_idx {
            self.items[idx].due_date = Some(date);
            self.items[idx].repeat = repeat;
            self.editing_idx = None;
        } else {
            let text = self.input.trim().to_string();
            let mut todo = Todo::new(&text);
            todo.due_date = Some(date);
            todo.repeat = repeat;
            self.items.push(todo);
            self.selected = self.items.len() - 1;
            self.input.clear();
        }
        self.mode = Mode::Normal;
    }
}
