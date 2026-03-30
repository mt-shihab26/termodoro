use std::cell::RefCell;
use std::io::Result;

use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::widgets::{Block, Clear, List, ListItem, ListState, Paragraph, Widget};

use crate::domains::todos::{Mode, TodosState};
use crate::handlers::tui::widgets::calendar_popup::CalendarPopup;

use super::Tab;

const REPEAT_OPTIONS: &[&str] = &[
    "None",
    "Daily",
    "Weekly (same day)",
    "Weekdays (Mon-Fri)",
    "Monthly on day",
    "Yearly on day",
];

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

    fn render_repeat_popup(&self, frame: &mut Frame, area: Rect) {
        let popup_w = 30u16;
        let popup_h = (REPEAT_OPTIONS.len() as u16) + 3; // border + hint
        let popup = centered_rect(area, popup_w, popup_h);

        frame.render_widget(Clear, popup);

        let block = Block::bordered()
            .title(" Repeat ")
            .border_style(Style::default().fg(Color::Cyan));
        let inner = block.inner(popup);
        frame.render_widget(block, popup);

        let [list_area, hint_area] = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(inner);

        let items: Vec<ListItem> = REPEAT_OPTIONS
            .iter()
            .enumerate()
            .map(|(i, &opt)| {
                let selected = i == self.state.repeat_cursor;
                let prefix = if selected { ">" } else { " " };
                let style = if selected {
                    Style::default().fg(Color::Cyan).bold()
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(format!("{} {}", prefix, opt)).style(style)
            })
            .collect();

        frame.render_widget(List::new(items), list_area);
        frame.render_widget(
            Paragraph::new("[j/k]Navigate  [Enter]Select  [Esc]Skip")
                .centered()
                .fg(Color::DarkGray),
            hint_area,
        );
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
            Mode::SelectingDate => "Calendar — navigate to pick a due date",
            Mode::SelectingRepeat => "Repeat — choose how often this todo recurs",
        };
        frame.render_widget(Paragraph::new(hint).centered().fg(Color::DarkGray), hint_area);

        if let Some(area) = input_area {
            let block = Block::bordered()
                .title(" New Todo ")
                .border_style(Style::default().fg(self.color()));
            let inner = block.inner(area);
            frame.render_widget(block, area);
            frame.render_widget(Paragraph::new(format!("{}_", self.state.input)).fg(Color::White), inner);
        }

        // Overlay popups
        match self.state.mode {
            Mode::SelectingDate => frame.render_widget(
                CalendarPopup::new(self.state.calendar_date, self.state.calendar_view),
                area,
            ),
            Mode::SelectingRepeat => self.render_repeat_popup(frame, area),
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
                KeyCode::Enter => self.state.confirm_date(),
                KeyCode::Esc => self.state.skip_date(),
                _ => {}
            },
            Mode::SelectingRepeat => match key.code {
                KeyCode::Char('j') | KeyCode::Down => self.state.repeat_move_down(),
                KeyCode::Char('k') | KeyCode::Up => self.state.repeat_move_up(),
                KeyCode::Enter => self.state.confirm_repeat(),
                KeyCode::Esc => self.state.skip_repeat(),
                _ => {}
            },
        }
        self.sync_list_state();
        Ok(())
    }
}

fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    let x = area.x + area.width.saturating_sub(width) / 2;
    let y = area.y + area.height.saturating_sub(height) / 2;
    Rect {
        x,
        y,
        width: width.min(area.width),
        height: height.min(area.height),
    }
}
