use std::cell::RefCell;
use std::io::Result;

use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::widgets::{Block, List, ListItem, ListState, Paragraph, Widget};

use crate::domains::todos::{Mode, TodosState};
use crate::handlers::tui::widgets::calendar_popup::CalendarPopup;

use super::Tab;

pub struct Todos {
    state: TodosState,
    list: RefCell<ListState>,
}

impl Todos {
    pub fn new() -> Self {
        let state = TodosState::new();
        let mut list_state = ListState::default();
        list_state.select(Some(state.selected));
        Self {
            state,
            list: RefCell::new(list_state),
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
            Mode::Normal | Mode::SelectingDate | Mode::SelectingRepeat => {
                let [list, hint] =
                    Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(area);
                (list, hint, None)
            }
            Mode::Adding => {
                let [list, hint, input] = Layout::vertical([
                    Constraint::Fill(1),
                    Constraint::Length(1),
                    Constraint::Length(3),
                ])
                .areas(area);
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
            Mode::SelectingDate | Mode::SelectingRepeat => "",
        };
        frame.render_widget(Paragraph::new(hint).centered().fg(Color::DarkGray), hint_area);

        if let Some(area) = input_area {
            let block = Block::bordered()
                .title(" New Todo ")
                .border_style(Style::default().fg(self.color()));
            let inner = block.inner(area);
            frame.render_widget(block, area);
            frame.render_widget(
                Paragraph::new(format!("{}_", self.state.input)).fg(Color::White),
                inner,
            );
        }

        // Calendar popup overlay (also shows repeat section when in SelectingRepeat)
        match self.state.mode {
            Mode::SelectingDate => frame.render_widget(
                CalendarPopup::new(self.state.calendar_date, self.state.calendar_view),
                area,
            ),
            Mode::SelectingRepeat => frame.render_widget(
                CalendarPopup::with_repeat(
                    self.state.calendar_date,
                    self.state.calendar_view,
                    self.state.repeat_cursor,
                ),
                area,
            ),
            _ => {}
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
                KeyCode::Char('e') => self.state.start_edit_date(),
                _ => {}
            },
            Mode::Adding => match key.code {
                KeyCode::Enter => self.state.confirm_add(),
                KeyCode::Esc => self.state.cancel_add(),
                KeyCode::Backspace => {
                    self.state.input.pop();
                }
                KeyCode::Char(c) => self.state.input.push(c),
                _ => {}
            },
            Mode::SelectingDate => match key.code {
                KeyCode::Char('h') | KeyCode::Left => self.state.calendar_nav_left(),
                KeyCode::Char('l') | KeyCode::Right => self.state.calendar_nav_right(),
                KeyCode::Char('j') | KeyCode::Down => self.state.calendar_nav_down(),
                KeyCode::Char('k') | KeyCode::Up => self.state.calendar_nav_up(),
                KeyCode::Char('H') => self.state.calendar_prev_month(),
                KeyCode::Char('L') => self.state.calendar_next_month(),
                KeyCode::Char('t') => self.state.set_date_today(),
                KeyCode::Char('y') => self.state.set_date_yesterday(),
                KeyCode::Char('n') => self.state.set_date_tomorrow(),
                KeyCode::Char('r') => self.state.open_repeat(),
                KeyCode::Enter => self.state.confirm_date(),
                KeyCode::Esc => self.state.cancel_selecting_date(),
                _ => {}
            },
            Mode::SelectingRepeat => match key.code {
                KeyCode::Char('j') | KeyCode::Down => self.state.repeat_move_down(),
                KeyCode::Char('k') | KeyCode::Up => self.state.repeat_move_up(),
                KeyCode::Enter => self.state.confirm_repeat(),
                KeyCode::Esc => self.state.cancel_repeat(),
                _ => {}
            },
        }
        self.sync_list_state();
        Ok(())
    }
}
