# Widget Pattern

Every UI component follows a three-part split: **Props → State → Widget**.

---

## The three parts

### Props

Holds the data a widget needs to render one frame. It is a plain struct
with no behaviour — just fields. The widget borrows `&Props`, it never
owns or mutates it.

```rust
pub struct MyProps {
    pub value: u64,
}
```

### State

Owns the runtime data that changes over time. Lives in the caller
(a tab, `App`, or a parent component) — never inside the widget.
Exposes a `props` field so the caller can hand `&state.props` to the
widget at render time without cloning.

```rust
pub struct MyState {
    pub props: MyProps,
    // private tracking fields ...
}

impl MyState {
    pub fn tick(&mut self) { /* update props */ }
}
```

### Widget

A stateless view. Created inline at render time, never stored. Borrows
`&Props` for the duration of one render call, then is dropped.

```rust
pub struct MyWidget<'a> {
    props: &'a MyProps,
}

impl<'a> MyWidget<'a> {
    pub fn new(props: &'a MyProps) -> Self { Self { props } }
}

impl Widget for &MyWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) { ... }
}
```

---

## How the caller wires it together

```rust
// 1. Own the state
my_state: MyState,

// 2. Update it before drawing
self.my_state.tick();

// 3. Pass props by reference into the widget at render time
MyWidget::new(&self.my_state.props).render(area, buf);
```

---

## Rules

- **State is never passed to the widget** — only `&props`.
- **Widgets are never stored** — created and dropped each frame.
- **Visibility is the caller's concern** — wrap the render call in an
  `if` instead of adding a flag inside the widget.
- **Implement `Widget for &MyWidget`** (shared ref) unless the widget
  must mutate itself during render, in which case use `&mut MyWidget`.

---

## When to simplify

| Situation | Simplification |
|---|---|
| No changing state | Skip `State`, build `Props` inline in the caller |
| All fields are `Copy` | Derive `Copy` on `Props`, pass by value, no lifetime needed |
| Single render-time value | Pass the field directly, skip a `Props` struct entirely |
