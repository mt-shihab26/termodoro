use time::Date;

use super::Repeat;

pub struct Todo {
    pub text: String,
    pub done: bool,
    pub due_date: Option<Date>,
    pub repeat: Repeat,
}

impl Todo {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            done: false,
            due_date: None,
            repeat: Repeat::None,
        }
    }

    pub fn fakes() -> Vec<Todo> {
        vec![
            Todo::new("Buy groceries"),
            Todo::new("Read a book"),
            Todo::new("Build a TUI app"),
        ]
    }
}
