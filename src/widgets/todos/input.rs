use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    prelude::{Buffer, Color, Constraint, Frame, Layout, Rect, Style, Stylize, Widget},
    text::Span,
    widgets::{Block, Paragraph},
};
use ratatui_textarea::TextArea;
use time::Date;

use crate::{kinds::repeat::Repeat, tabs::todos::COLOR};

use super::calendar::{CalendarAction, CalendarProps, CalendarState, CalendarWidget};

/// Action returned by the input widget after handling a key event.
pub enum InputAction {
    /// User submitted the input; carries the text, optional due date, and repeat rule.
    Confirm {
        /// The trimmed text entered by the user.
        text: String,
        /// Optional due date chosen via the calendar picker.
        date: Option<Date>,
        /// Optional repeat rule chosen via the repeat picker.
        repeat: Option<Repeat>,
    },
    /// User pressed Escape to cancel input.
    Escape,
    /// No state change occurred.
    None,
}

/// Props for the todo text-input widget.
pub struct InputProps {
    /// The textarea holding the user's current text input.
    textarea: TextArea<'static>,
    /// Currently selected due date, if any.
    date: Option<Date>,
    /// Currently selected repeat rule, if any.
    repeat: Option<Repeat>,
}

impl InputProps {
    /// Creates new input props, optionally pre-filling text, date, and repeat.
    pub fn new(text: Option<&str>, date: Option<Date>, repeat: Option<&Repeat>) -> Self {
        let mut textarea = TextArea::default();
        if let Some(t) = text {
            textarea.insert_str(t);
        }
        textarea.set_block(Block::bordered().border_style(Style::default().fg(COLOR)));
        textarea.set_cursor_line_style(Style::default());
        Self {
            textarea,
            date,
            repeat: repeat.map(Repeat::of),
        }
    }
}

/// Stateful container for the todo input, owns props and optional calendar overlay.
pub struct InputState {
    /// Mutable props updated as the user types or picks a date/repeat.
    props: InputProps,
    /// Active calendar state when the date-picker overlay is open.
    calendar_state: Option<CalendarState>,
}

impl InputState {
    /// Creates a new input state wrapping the given props.
    pub fn new(props: InputProps) -> Self {
        Self {
            props,
            calendar_state: None,
        }
    }

    /// Returns a shared reference to the current props.
    pub fn props(&self) -> &InputProps {
        &self.props
    }

    /// Handles a key event, delegating to the calendar overlay when open.
    pub fn handle(&mut self, key: KeyEvent) -> InputAction {
        if let Some(cal) = &mut self.calendar_state {
            match cal.handle(key) {
                CalendarAction::Confirm { date, repeat } => {
                    self.props.date = date;
                    self.props.repeat = repeat;
                    self.calendar_state = None;
                }
                CalendarAction::Cancel => self.calendar_state = None,
                CalendarAction::None => {}
            }
            return InputAction::None;
        }

        match key.code {
            KeyCode::Enter => {
                let text = self.props.textarea.lines()[0].clone();
                if !text.trim().is_empty() {
                    return InputAction::Confirm {
                        text,
                        date: self.props.date,
                        repeat: self.props.repeat.as_ref().map(Repeat::of),
                    };
                }
            }
            KeyCode::Esc => return InputAction::Escape,
            KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.calendar_state = Some(CalendarState::new(CalendarProps::new(
                    self.props.date,
                    self.props.repeat.as_ref(),
                )));
            }
            _ => {
                self.props.textarea.input(key);
            }
        }
        InputAction::None
    }

    /// Renders the calendar overlay into the frame if it is currently open.
    pub fn render_calendar(&self, frame: &mut Frame, area: Rect) {
        if let Some(cal) = &self.calendar_state {
            frame.render_widget(&CalendarWidget::new(cal.props()), area);
        }
    }
}

/// Stateless widget that renders the todo text-input row.
pub struct InputWidget<'a> {
    /// Borrowed input props for this render pass.
    props: &'a InputProps,
}

impl<'a> InputWidget<'a> {
    /// Creates a new input widget from the given props.
    pub fn new(props: &'a InputProps) -> Self {
        Self { props }
    }
}

impl Widget for &InputWidget<'_> {
    /// Renders the textarea alongside the due-date block into the buffer.
    fn render(self, area: Rect, buf: &mut Buffer) {
        let date_area_width = self
            .props
            .repeat
            .as_ref()
            .map(|r| (r.label().len() as u16 + 4).max(14))
            .unwrap_or(14);

        let icon_width = if self.props.repeat.is_some() { 2 } else { 0 };
        let [icon_area, text_area, date_area] = Layout::horizontal([
            Constraint::Length(icon_width),
            Constraint::Fill(1),
            Constraint::Length(date_area_width),
        ])
        .areas(area);

        if self.props.repeat.is_some() {
            let v_offset = icon_area.height / 2;
            let centered = Rect {
                y: icon_area.y + v_offset,
                height: 1,
                ..icon_area
            };
            Paragraph::new(Repeat::icon())
                .fg(COLOR)
                .bold()
                .centered()
                .render(centered, buf);
        }

        Widget::render(&self.props.textarea, text_area, buf);

        let date_str = match self.props.date {
            Some(d) => format!("{}", d),
            None => "no date".to_string(),
        };

        let mut block = Block::bordered()
            .title(Span::from(" ^d ").fg(Color::DarkGray).bold())
            .border_style(Style::default().fg(COLOR));
        if let Some(repeat) = self.props.repeat.as_ref() {
            block = block.title_bottom(Span::from(format!(" {} ", repeat.label())).fg(COLOR).bold());
        }
        let inner = block.inner(date_area);
        block.render(date_area, buf);

        Paragraph::new(date_str).fg(COLOR).centered().render(inner, buf);
    }
}
