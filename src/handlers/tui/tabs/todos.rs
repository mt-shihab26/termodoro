use std::cell::RefCell;
use std::io::Result;

use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::widgets::{Block, List, ListItem, ListState, Paragraph, Widget};

use crate::domains::todos::TodosState;
use crate::handlers::tui::widgets::input_area::{InputArea, InputAreaAction};

use super::Tab;

#[derive(PartialEq)]
pub enum UiMode {
    Normal,
    Adding,
    Editing,
}

pub struct Todos {
    state: TodosState,
    ui_mode: UiMode,
    selected_item_index: usize,
    adding_input_area: InputArea,
    editing_input_area: InputArea,
    list_state: RefCell<ListState>,
}

impl Todos {
    pub fn new() -> Self {
        let state = TodosState::new();
        let mut list_state = ListState::default();
        list_state.select(Some(state.selected));

        Self {
            state,
            ui_mode: UiMode::Normal,
            selected_item_index: 0,
            adding_input_area: InputArea::new(None),
            editing_input_area: InputArea::new(None),
            list_state: RefCell::new(list_state),
        }
    }

    fn sync_list_state(&self) {
        let selected = if self.state.items.is_empty() {
            None
        } else {
            Some(self.state.selected)
        };
        self.list_state.borrow_mut().select(selected);
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

        let (list_area, hint_area, input_area) = match self.ui_mode {
            UiMode::Normal => {
                let [list, hint] = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(area);
                (list, hint, None)
            }
            UiMode::Adding | UiMode::Editing => {
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

        frame.render_stateful_widget(list, list_area, &mut self.list_state.borrow_mut());

        let hint = match self.ui_mode {
            UiMode::Normal => "[j/k]Navigate  [Space]Toggle  [a]Add  [d]Delete  [e]Edit Date",
            UiMode::Adding | UiMode::Editing => "[Enter]Confirm  [Esc]Cancel  [Backspace]Delete char",
        };

        frame.render_widget(Paragraph::new(hint).centered().fg(Color::DarkGray), hint_area);

        match self.ui_mode {
            UiMode::Normal => {}
            UiMode::Adding => {
                if let Some(area) = input_area {
                    frame.render_widget(&self.adding_input_area, area);
                }
            }
            UiMode::Editing => {
                if let Some(area) = input_area {
                    frame.render_widget(&self.editing_input_area, area);
                }
            }
        }
    }

    fn handle(&mut self, key: KeyEvent) -> Result<()> {
        match self.ui_mode {
            UiMode::Normal => match key.code {
                KeyCode::Char(' ') | KeyCode::Enter => {
                    if !self.state.items.is_empty() {
                        self.state.items[self.selected_item_index].done =
                            !self.state.items[self.selected_item_index].done;
                    }
                }
                KeyCode::Char('j') | KeyCode::Down => {
                    if !self.state.items.is_empty() {
                        self.selected_item_index = (self.selected_item_index + 1).min(self.state.items.len() - 1);
                    }
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    self.selected_item_index = self.selected_item_index.saturating_sub(1);
                }
                KeyCode::Char('d') => {
                    if !self.state.items.is_empty() {
                        self.state.items.remove(self.selected_item_index);
                        if !self.state.items.is_empty() {
                            self.selected_item_index = self.selected_item_index.min(self.state.items.len() - 1);
                        } else {
                            self.selected_item_index = 0;
                        }
                    }
                }
                KeyCode::Char('a') => {
                    self.ui_mode = UiMode::Adding;
                    self.adding_input_area = InputArea::new(None)
                }
                KeyCode::Char('e') => {
                    if !self.state.items.is_empty() {
                        self.ui_mode = UiMode::Editing;
                        let todo = &self.state.items[self.selected_item_index];
                        self.editing_input_area = InputArea::new(Some(&todo.text))
                    }
                }
                _ => {}
            },
            UiMode::Adding => {
                match self.adding_input_area.handle(key) {
                    InputAreaAction::Confirm(text) => {
                        self.state.add(text);
                        self.adding_input_area = InputArea::new(None)
                    }
                    InputAreaAction::None => {}
                };
            }
            UiMode::Editing => {
                //
            }
        }
        self.sync_list_state();
        Ok(())
    }
}
