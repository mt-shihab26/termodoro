use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::{Buffer, Rect, Widget};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::Span;
use ratatui::widgets::{Block, Paragraph};
use ratatui_textarea::TextArea;
use time::Date;

use crate::domains::todos::repeat::Repeat;
use crate::handlers::tui::tabs::todos::COLOR;
use crate::handlers::tui::widgets::calendar_popup::{CalendarAction, CalendarPopup};

pub enum InputAreaAction {
    Confirm { text: String, date: Option<Date>, repeat: Option<Repeat> },
    Escape,
    None,
}

pub struct InputArea {
    textarea: TextArea<'static>,
    date: Option<Date>,
    repeat: Option<Repeat>,
    calendar: Option<CalendarPopup>,
}

impl InputArea {
    pub fn new(text: Option<&str>, date: Option<Date>, repeat: Option<Repeat>) -> Self {
        let mut textarea = TextArea::default();
        if let Some(t) = text {
            textarea.insert_str(t);
        }
        textarea.set_block(Block::bordered().border_style(Style::default().fg(COLOR)));
        textarea.set_cursor_line_style(Style::default());
        Self { textarea, date, repeat, calendar: None }
    }

    pub fn handle(&mut self, key: KeyEvent) -> InputAreaAction {
        if let Some(cal) = &mut self.calendar {
            match cal.handle(key) {
                CalendarAction::Confirm { date, repeat } => {
                    self.date = date;
                    self.repeat = repeat;
                    self.calendar = None;
                }
                CalendarAction::Cancel => self.calendar = None,
                CalendarAction::None => {}
            }
            return InputAreaAction::None;
        }

        match key.code {
            KeyCode::Enter => {
                let text = self.textarea.lines()[0].clone();
                if !text.trim().is_empty() {
                    return InputAreaAction::Confirm { text, date: self.date, repeat: self.repeat };
                }
            }
            KeyCode::Esc => return InputAreaAction::Escape,
            KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.calendar = Some(CalendarPopup::new(self.date, self.repeat));
            }
            _ => {
                self.textarea.input(key);
            }
        }
        InputAreaAction::None
    }

    pub fn render_calendar(&self, frame: &mut Frame, area: Rect) {
        if let Some(cal) = &self.calendar {
            frame.render_widget(cal, area);
        }
    }
}

impl Widget for &InputArea {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [text_area, date_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Length(14)]).areas(area);

        Widget::render(&self.textarea, text_area, buf);

        let date_str = match self.date {
            Some(d) => format!("{}", d),
            None => "no date".to_string(),
        };

        let block = Block::bordered()
            .title(Span::from(" ^d ").fg(Color::DarkGray).bold())
            .border_style(Style::default().fg(COLOR));
        let inner = block.inner(date_area);
        block.render(date_area, buf);

        Paragraph::new(date_str).fg(Color::DarkGray).centered().render(inner, buf);
    }
}
