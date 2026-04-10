use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Layout},
    prelude::{Buffer, Color, Rect, Style, Stylize, Widget},
    widgets::{Block, Clear, Paragraph},
};

/// Action returned by the reduce-picker after handling a key event.
pub enum ReducePickerAction {
    /// User confirmed; carries the number of milliseconds to subtract.
    Reduce(u32),
    /// User cancelled without applying.
    Cancel,
    /// No state change occurred.
    None,
}

/// Props for the reduce-picker overlay — plain data, no behaviour.
pub struct ReducePickerProps {
    /// Up to four digits entered so far: [tens-min, units-min, tens-sec, units-sec].
    digits: Vec<u8>,
    /// Pass the phase color to use in everywhere
    color: Color,
}

impl ReducePickerProps {
    /// Creates empty props with no digits entered.
    pub fn new(color: Color) -> Self {
        Self {
            digits: Vec::with_capacity(4),
            color,
        }
    }
}

/// Stateful container for the reduce-picker, owns its props.
pub struct ReducePickerState {
    /// Mutable props updated as the user types.
    props: ReducePickerProps,
}

impl ReducePickerState {
    /// Creates a new picker state with an empty digit buffer.
    pub fn new(props: ReducePickerProps) -> Self {
        Self { props }
    }

    /// Returns a shared reference to the current props.
    pub fn props(&self) -> &ReducePickerProps {
        &self.props
    }

    /// Handles a key event and returns the resulting action.
    pub fn handle(&mut self, key: KeyEvent) -> ReducePickerAction {
        match key.code {
            KeyCode::Char(c) if c.is_ascii_digit() => {
                if self.props.digits.len() < 4 {
                    let d = c as u8 - b'0';
                    // Tens-of-seconds digit (index 2) must be 0–5.
                    if self.props.digits.len() == 2 && d > 5 {
                        return ReducePickerAction::None;
                    }
                    self.props.digits.push(d);
                }
                ReducePickerAction::None
            }
            KeyCode::Backspace => {
                self.props.digits.pop();
                ReducePickerAction::None
            }
            KeyCode::Enter => {
                if self.props.digits.is_empty() {
                    return ReducePickerAction::Cancel;
                }
                let get = |i: usize| *self.props.digits.get(i).unwrap_or(&0) as u32;
                let millis = (get(0) * 10 + get(1)) * 60_000 + (get(2) * 10 + get(3)) * 1_000;
                ReducePickerAction::Reduce(millis)
            }
            KeyCode::Esc => ReducePickerAction::Cancel,
            _ => ReducePickerAction::None,
        }
    }
}

/// Stateless widget that renders the reduce-time popup.
pub struct ReducePickerWidget<'a> {
    /// Borrowed picker props for this render pass.
    props: &'a ReducePickerProps,
}

impl<'a> ReducePickerWidget<'a> {
    /// Creates a new widget from the given props.
    pub fn new(props: &'a ReducePickerProps) -> Self {
        Self { props }
    }
}

impl Widget for &ReducePickerWidget<'_> {
    /// Renders the centered popup into the buffer.
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup = centered_rect(area, 36, 7);

        Clear.render(popup, buf);

        let block = Block::bordered()
            .title(" Reduce Remaining ")
            .title_bottom(" [Enter] Apply  [Esc] Cancel ")
            .border_style(Style::default().fg(self.props.color).bold());

        let inner = block.inner(popup);
        block.render(popup, buf);

        let [_, display_row, _, label_row, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .areas(inner);

        let ch = |i: usize| self.props.digits.get(i).map(|d| (b'0' + d) as char).unwrap_or('_');
        let display = format!("{}{}:{}{}", ch(0), ch(1), ch(2), ch(3));

        Paragraph::new(display)
            .centered()
            .bold()
            .fg(self.props.color)
            .render(display_row, buf);

        Paragraph::new("minutes : seconds")
            .centered()
            .fg(Color::DarkGray)
            .render(label_row, buf);
    }
}

/// Computes a centered popup rect of the given dimensions within `area`.
fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    Rect {
        x: area.x + area.width.saturating_sub(width) / 2,
        y: area.y + area.height.saturating_sub(height) / 2,
        width: width.min(area.width),
        height: height.min(area.height),
    }
}
