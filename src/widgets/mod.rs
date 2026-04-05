//! # Widget Pattern
//!
//! Every UI component follows a three-part split: **Action(optional) → Props → State(optional) → Widget**.
//!
//! A complete real-world example of this pattern: [`crate::widgets::layout::fps`] or [`crate::widgets::timer::todo_picker`]
//!
//! ## The four parts
//!
//! ### Action *(optional)*
//!
//! An enum returned by `State::handle()` to communicate events back to the caller.
//! Only add this when the widget must signal something upward (selection, cancellation, etc.).
//! If the widget is purely visual with no input handling, skip `Action` and `handle` entirely.
//!
//! ```rust,ignore
//! pub enum MyAction {
//!     Select((i32, String)),
//!     Cancel,
//!     None,
//! }
//! ```
//!
//! ### Props
//!
//! Holds the data a widget needs to render one frame.
//! It is a plain struct with no behaviour — just fields.
//! The widget borrows `&Props`, it never owns or mutates it.
//!
//! ```rust,ignore
//! pub struct MyProps {
//!     items: Vec<(i32, String)>,
//!     cursor: usize,
//! }
//!
//! impl MyProps {
//!     pub fn new(items: Vec<(i32, String)>) -> Self {
//!         Self { items, cursor: 0 }
//!     }
//! }
//! ```
//!
//! ### State *(optional)*
//!
//! Owns the runtime data that changes over time.
//! Lives in the caller (a parent component) — never inside the widget.
//! Exposes a `props()` getter so the caller can hand `&props` to the widget at render time without cloning.
//!
//! If the widget is stateless (no runtime data to track), skip `State` and construct `Props` directly in the caller.
//! Optionally exposes a `handle()` method that processes input and returns an `Action`.
//!
//! ```rust,ignore
//! pub struct MyState {
//!     props: MyProps,  // private
//! }
//!
//! impl MyState {
//!     pub fn new(props: MyProps) -> Self {
//!         Self { props }
//!     }
//!
//!     pub fn props(&self) -> &MyProps {
//!         &self.props
//!     }
//!
//!     // optional — only add if the widget handles input
//!     pub fn handle(&mut self, key: KeyEvent) -> MyAction {
//!         match key.code {
//!             KeyCode::Enter => { /* ... */ MyAction::Select((0, String::new())) }
//!             KeyCode::Esc   => MyAction::Cancel,
//!             _              => MyAction::None,
//!         }
//!     }
//! }
//! ```
//!
//! ### Widget
//!
//! A stateless view.
//! Created inline at render time, never stored.
//! Borrows `&Props` for the duration of one render call, then is dropped.
//!
//! ```rust,ignore
//! pub struct MyWidget<'a> {
//!     props: &'a MyProps,
//! }
//!
//! impl<'a> MyWidget<'a> {
//!     pub fn new(props: &'a MyProps) -> Self { Self { props } }
//! }
//!
//! impl Widget for &MyWidget<'_> {
//!     fn render(self, area: Rect, buf: &mut Buffer) { ... }
//! }
//! ```
//!
//! ---
//!
//! ## How the caller wires it together
//!
//! ### With state and input handling
//!
//! ```rust,ignore
//! // 1. Own the state
//! my_state: MyState,
//!
//! // 2. Handle input
//! let action = self.my_state.handle(key_event);
//! match action {
//!     MyAction::Select(item) => { /* do something with item */ }
//!     MyAction::Cancel       => { /* dismiss the widget */ }
//!     MyAction::None         => {}
//! }
//!
//! // 3. Pass props via getter into the widget at render time
//! MyWidget::new(self.my_state.props()).render(area, buf);
//! ```
//!
//! ### Stateless widget (no State, no Action)
//!
//! ```rust,ignore
//! // 1. Construct props directly — no state needed
//! let props = MyProps::new(items);
//!
//! // 2. Pass props straight into the widget at render time
//! MyWidget::new(&props).render(area, buf);
//! ```
//!
//! ---
//!
//! ## Rules
//!
//! - **State is never passed to the widget** — only `&props`.
//! - **Widgets are never stored** — created and dropped each frame.
//! - **Visibility is the caller's concern** — wrap the render call in an `if` instead of adding a flag inside the widget.
//! - **Implement `Widget for &MyWidget`** — always use a shared ref.
//! - **`State` is optional** — if the widget has no runtime data to track, construct `Props` directly in the caller and skip `State`.
//! - **`Action` and `handle` are optional** — add them only when the widget handles input and needs to signal events to the caller.

/// Shared layout widgets such as borders and FPS counters.
pub mod layout;
/// Widgets used by the timer tab and its overlays.
pub mod timer;
/// Widgets used by the todos tab and its overlays.
pub mod todos;
