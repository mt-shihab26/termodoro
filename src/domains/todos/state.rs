use super::todo::Todo;

pub enum Mode {
    Normal,
    Adding,
}

pub struct TodosState {
    pub items: Vec<Todo>,
    pub selected: usize,
    pub mode: Mode,
    pub input: String,
}

impl TodosState {
    pub fn new() -> Self {
        Self {
            items: Todo::fakes(),
            selected: 0,
            mode: Mode::Normal,
            input: String::new(),
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

    pub fn confirm_add(&mut self) {
        let text = self.input.trim();
        if !text.is_empty() {
            self.items.push(Todo::new(text));
            self.selected = self.items.len() - 1;
        }
        self.mode = Mode::Normal;
        self.input.clear();
    }

    pub fn cancel_add(&mut self) {
        self.mode = Mode::Normal;
        self.input.clear();
    }
}
