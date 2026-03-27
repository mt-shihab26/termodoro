pub struct Todo {
    pub text: String,
    pub done: bool,
}

impl Todo {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            done: false,
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
