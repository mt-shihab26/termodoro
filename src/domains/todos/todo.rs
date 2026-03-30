use time::Date;

use super::repeat::Repeat;

pub struct Todo {
    pub text: String,
    pub done: bool,
    pub due_date: Option<Date>,
    pub repeat: Option<Repeat>,
}

impl Todo {
    pub fn new(text: String, due_date: Option<Date>, repeat: Option<Repeat>) -> Self {
        Self {
            text,
            done: false,
            due_date,
            repeat,
        }
    }

    fn from_text(text: &str) -> Self {
        Self {
            text: text.to_string(),
            done: false,
            due_date: None,
            repeat: None,
        }
    }

    pub fn fakes() -> Vec<Todo> {
        vec![
            Todo::from_text("Buy groceries"),
            Todo::from_text("Read a book"),
            Todo::from_text("Build a TUI app"),
        ]
    }
}
