/// Current UI mode for the todos tab.
#[derive(Clone, Copy)]
pub enum TodosMode {
    /// Default browsing mode for navigating and acting on todos.
    Normal,
    /// Input mode for creating a new todo.
    Adding,
    /// Input mode for editing the selected todo.
    Editing,
    /// Search mode for filtering todos by text on the current page.
    Searching,
}
