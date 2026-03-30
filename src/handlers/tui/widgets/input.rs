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
use crate::handlers::tui::widgets::calendar::{CalendarAction, CalendarWidget};

pub enum InputAction {
    Confirm {
        text: String,
        date: Option<Date>,
        repeat: Option<Repeat>,
    },
    Escape,
    None,
}

pub struct InputWidget {
    textarea: TextArea<'static>,
    date: Option<Date>,
    repeat: Option<Repeat>,
    calendar_widget: Option<CalendarWidget>,
}

impl InputWidget {
    pub fn new(text: Option<&str>, date: Option<Date>, repeat: Option<Repeat>) -> Self {
        let mut textarea = TextArea::default();
        if let Some(t) = text {
            textarea.insert_str(t);
        }
        textarea.set_block(Block::bordered().border_style(Style::default().fg(COLOR)));
        textarea.set_cursor_line_style(Style::default());
        Self {
            textarea,
            date,
            repeat,
            calendar_widget: None,
        }
    }

    pub fn handle(&mut self, key: KeyEvent) -> InputAction {
        if let Some(cal) = &mut self.calendar_widget {
            match cal.handle(key) {
                CalendarAction::Confirm { date, repeat } => {
                    self.date = date;
                    self.repeat = repeat;
                    self.calendar_widget = None;
                }
                CalendarAction::Cancel => self.calendar_widget = None,
                CalendarAction::None => {}
            }
            return InputAction::None;
        }

        match key.code {
            KeyCode::Enter => {
                let text = self.textarea.lines()[0].clone();
                if !text.trim().is_empty() {
                    return InputAction::Confirm {
                        text,
                        date: self.date,
                        repeat: self.repeat,
                    };
                }
            }
            KeyCode::Esc => return InputAction::Escape,
            KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.calendar_widget = Some(CalendarWidget::new(self.date, self.repeat));
            }
            _ => {
                self.textarea.input(key);
            }
        }
        InputAction::None
    }

    pub fn render_calendar(&self, frame: &mut Frame, area: Rect) {
        if let Some(cal) = &self.calendar_widget {
            frame.render_widget(cal, area);
        }
    }
}

impl Widget for &InputWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [text_area, date_area] = Layout::horizontal([Constraint::Fill(1), Constraint::Length(14)]).areas(area);

        Widget::render(&self.textarea, text_area, buf);

        let date_str = match self.date {
            Some(d) => format!("{}", d),
            None => "no date".to_string(),
        };

        let mut block = Block::bordered()
            .title(Span::from(" ^d ").fg(Color::DarkGray).bold())
            .border_style(Style::default().fg(COLOR));
        if let Some(ref repeat) = self.repeat {
            block = block.title_bottom(Span::from(format!(" {} ", repeat.label())).fg(COLOR).bold());
        }
        let inner = block.inner(date_area);
        block.render(date_area, buf);

        Paragraph::new(date_str)
            .fg(Color::DarkGray)
            .centered()
            .render(inner, buf);
    }
}
