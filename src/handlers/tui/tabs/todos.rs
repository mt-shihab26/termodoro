use std::cell::RefCell;
use std::io::Result;

use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::widgets::{Block, List, ListItem, ListState, Paragraph, Widget};

use crate::domains::todos::{Mode, TodosState};
use crate::handlers::tui::widgets::calendar_popup::{CalendarAction, CalendarPopup};
use crate::handlers::tui::widgets::input_area::{InputArea, InputAreaAction};

use super::Tab;

pub struct Todos {
    state: TodosState,
    list: RefCell<ListState>,
    input: InputArea,
    calendar: CalendarPopup,
}

impl Todos {
    pub fn new() -> Self {
        let state = TodosState::new();
        let mut list_state = ListState::default();
        list_state.select(Some(state.selected));
        Self {
            state,
            list: RefCell::new(list_state),
            input: InputArea::new(),
            calendar: CalendarPopup::new(None, None),
        }
    }

    fn sync_list_state(&self) {
        let selected = if self.state.items.is_empty() {
            None
        } else {
            Some(self.state.selected)
        };
        self.list.borrow_mut().select(selected);
    }
}

impl Tab for Todos {
    fn name(&self) -> &str {
        "Todos [1]"
    }

    fn color(&self) -> Color {
        Color::Cyan
    }

    fn render(&self, frame: &mut Frame, area: Rect) {
        let buf = frame.buffer_mut();

        let block = Block::bordered().fg(self.color());
        let inner = block.inner(area);
        block.render(area, buf);

        let area = inner;

        let (list_area, hint_area, input_area) = match self.state.mode {
            Mode::Normal | Mode::SelectingDate => {
                let [list, hint] = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(area);
                (list, hint, None)
            }
            Mode::Adding => {
                let [list, hint, input] =
                    Layout::vertical([Constraint::Fill(1), Constraint::Length(1), Constraint::Length(3)]).areas(area);
                (list, hint, Some(input))
            }
        };

        let items: Vec<ListItem> = self
            .state
            .items
            .iter()
            .map(|todo| {
                let check = if todo.done { "[x]" } else { "[ ]" };
                let mut label = format!(" {} {}", check, todo.text);
                if let Some(date) = todo.due_date {
                    label.push_str(&format!("  [{}]", date));
                }
                if let Some(ref repeat) = todo.repeat {
                    label.push_str(&format!("  [{}]", repeat.label()));
                }
                let style = if todo.done {
                    Style::default().fg(Color::DarkGray).add_modifier(Modifier::CROSSED_OUT)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(label).style(style)
            })
            .collect();

        let list = List::new(items)
            .highlight_style(Style::default().fg(self.color()).bold())
            .highlight_symbol(">");

        frame.render_stateful_widget(list, list_area, &mut self.list.borrow_mut());

        let hint = match self.state.mode {
            Mode::Normal => "[j/k]Navigate  [Space]Toggle  [a]Add  [d]Delete  [e]Edit Date",
            Mode::Adding => "[Enter]Confirm  [Esc]Cancel  [Backspace]Delete char",
            Mode::SelectingDate => "",
        };
        frame.render_widget(Paragraph::new(hint).centered().fg(Color::DarkGray), hint_area);

        if let Some(area) = input_area {
            frame.render_widget(&self.input, area);
        }

        if self.state.mode == Mode::SelectingDate {
            frame.render_widget(&self.calendar, area);
        }
    }

    fn handle(&mut self, key: KeyEvent) -> Result<()> {
        match self.state.mode {
            Mode::Normal => match key.code {
                KeyCode::Char('j') | KeyCode::Down => self.state.move_down(),
                KeyCode::Char('k') | KeyCode::Up => self.state.move_up(),
                KeyCode::Char(' ') | KeyCode::Enter => self.state.toggle_selected(),
                KeyCode::Char('a') => self.state.start_adding(),
                KeyCode::Char('d') => self.state.delete_selected(),
                KeyCode::Char('e') => {
                    if !self.state.items.is_empty() {
                        let idx = self.state.selected;
                        self.calendar =
                            CalendarPopup::new(self.state.items[idx].due_date, self.state.items[idx].repeat);
                        self.state.start_edit_date();
                    }
                }
                _ => {}
            },
            Mode::Adding => {
                match self.input.handle(key) {
                    InputAreaAction::Confirm(text) => self.state.add(text),
                    InputAreaAction::None => {}
                };
            }
            Mode::SelectingDate => match self.calendar.handle(key) {
                CalendarAction::Confirm { date, repeat } => {
                    self.state.confirm_with(date, repeat);
                    self.calendar = CalendarPopup::new(None, None);
                }
                CalendarAction::Cancel => self.state.cancel_selecting_date(),
                CalendarAction::None => {}
            },
        }
        self.sync_list_state();
        Ok(())
    }
}
