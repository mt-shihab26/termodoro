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

pub enum InputAction {
    Confirm {
        text: String,
        date: Option<Date>,
        repeat: Option<Repeat>,
    },
    Escape,
    None,
}

pub struct InputProps {
    textarea: TextArea<'static>,
    date: Option<Date>,
    repeat: Option<Repeat>,
}

impl InputProps {
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

pub struct InputState {
    props: InputProps,
    calendar_state: Option<CalendarState>,
}

impl InputState {
    pub fn new(props: InputProps) -> Self {
        Self {
            props,
            calendar_state: None,
        }
    }

    pub fn props(&self) -> &InputProps {
        &self.props
    }

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

    pub fn render_calendar(&self, frame: &mut Frame, area: Rect) {
        if let Some(cal) = &self.calendar_state {
            frame.render_widget(&CalendarWidget::new(cal.props()), area);
        }
    }
}

pub struct InputWidget<'a> {
    props: &'a InputProps,
}

impl<'a> InputWidget<'a> {
    pub fn new(props: &'a InputProps) -> Self {
        Self { props }
    }
}

impl Widget for &InputWidget<'_> {
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
