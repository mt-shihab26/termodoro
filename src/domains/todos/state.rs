use time::Date;

use crate::domains::todos::Repeat;

use super::todo::Todo;

pub struct TodosState {
    pub items: Vec<Todo>,
    pub selected: usize,
    pub input: String,
    pub editing_idx: Option<usize>,
}

impl TodosState {
    pub fn new() -> Self {
        Self {
            items: Todo::fakes(),
            selected: 0,
            input: String::new(),
            editing_idx: None,
        }
    }

    pub fn add(&mut self, text: String) {
        self.items.push(Todo::new(&text));
    }

    /// Open the calendar to edit the due date of the selected todo.
    pub fn start_edit_date(&mut self) {
        if self.items.is_empty() {
            return;
        }
        self.editing_idx = Some(self.selected);
    }

    /// Discard the in-progress add/edit and return to Normal.
    pub fn cancel_selecting_date(&mut self) {
        if self.editing_idx.is_none() {
            self.input.clear();
        }
        self.editing_idx = None;
    }

    /// Commit the selected date and repeat to a new or existing todo.
    pub fn confirm_with(&mut self, date: Option<Date>, repeat: Option<Repeat>) {
        if let Some(idx) = self.editing_idx {
            self.items[idx].due_date = date;
            self.items[idx].repeat = repeat;
            self.editing_idx = None;
        } else {
            let text = self.input.trim().to_string();
            let mut todo = Todo::new(&text);
            todo.due_date = date;
            todo.repeat = repeat;
            self.items.push(todo);
            self.selected = self.items.len() - 1;
            self.input.clear();
        }
    }
}
