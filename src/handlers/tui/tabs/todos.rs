use std::cell::RefCell;
use std::io::Result;

use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::widgets::{Block, List, ListItem, ListState, Paragraph, Widget};

use crate::domains::todos::todo::Todo;
use crate::handlers::tui::widgets::input_widget::{InputWidget, InputWidgetAction};

use super::Tab;

pub const COLOR: Color = Color::Green;

pub enum UiMode {
    Normal,
    Adding,
    Editing,
}

pub struct Todos {
    items: Vec<Todo>,
    ui_mode: UiMode,
    selected: usize,
    list_state: RefCell<ListState>,
    input_widget: Option<InputWidget>,
}

impl Todos {
    pub fn new() -> Self {
        Self {
            items: Todo::fakes(),
            ui_mode: UiMode::Normal,
            selected: 0,
            list_state: RefCell::new(ListState::default()),
            input_widget: None,
        }
    }

    fn sync_list_state(&self) {
        let selected = if self.items.is_empty() {
            None
        } else {
            Some(self.selected)
        };

        self.list_state.borrow_mut().select(selected);
    }
}

impl Tab for Todos {
    fn name(&self) -> &str {
        "Todos [^t]"
    }

    fn color(&self) -> Color {
        COLOR
    }

    fn handle(&mut self, key: KeyEvent) -> Result<()> {
        match self.ui_mode {
            UiMode::Normal => match key.code {
                KeyCode::Char(' ') | KeyCode::Enter => {
                    if !self.items.is_empty() {
                        self.items[self.selected].done = !self.items[self.selected].done;
                    }
                }
                KeyCode::Char('j') | KeyCode::Down => {
                    if !self.items.is_empty() {
                        self.selected = (self.selected + 1).min(self.items.len() - 1);
                    }
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    self.selected = self.selected.saturating_sub(1);
                }
                KeyCode::Char('d') => {
                    if !self.items.is_empty() {
                        self.items.remove(self.selected);
                        if !self.items.is_empty() {
                            self.selected = self.selected.min(self.items.len() - 1);
                        } else {
                            self.selected = 0;
                        }
                    }
                }
                KeyCode::Char('a') => {
                    self.ui_mode = UiMode::Adding;
                    self.input_widget = Some(InputWidget::new(None, None, None))
                }
                KeyCode::Char('e') => {
                    if !self.items.is_empty() {
                        self.ui_mode = UiMode::Editing;
                        let todo = &self.items[self.selected];
                        self.input_widget = Some(InputWidget::new(Some(&todo.text), todo.due_date, todo.repeat))
                    }
                }
                _ => {}
            },
            UiMode::Adding => {
                if let Some(input_widget) = &mut self.input_widget {
                    match input_widget.handle(key) {
                        InputWidgetAction::Confirm { text, date, repeat } => {
                            self.items.push(Todo {
                                text,
                                done: false,
                                due_date: date,
                                repeat,
                            });
                            self.input_widget = None;
                            self.ui_mode = UiMode::Normal;
                        }
                        InputWidgetAction::Escape => {
                            self.input_widget = None;
                            self.ui_mode = UiMode::Normal;
                        }
                        InputWidgetAction::None => {}
                    }
                }
            }
            UiMode::Editing => {
                if let Some(input_widget) = &mut self.input_widget {
                    match input_widget.handle(key) {
                        InputWidgetAction::Confirm { text, date, repeat } => {
                            self.items[self.selected].text = text;
                            self.items[self.selected].due_date = date;
                            self.items[self.selected].repeat = repeat;
                            self.input_widget = None;
                            self.ui_mode = UiMode::Normal;
                        }
                        InputWidgetAction::Escape => {
                            self.input_widget = None;
                            self.ui_mode = UiMode::Normal;
                        }
                        InputWidgetAction::None => {}
                    }
                }
            }
        }

        self.sync_list_state();

        Ok(())
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
            UiMode::Normal => "[j/k]Navigate  [Space]Toggle  [a]Add  [e]Edit  [d]Delete",
            UiMode::Adding | UiMode::Editing => "[Enter]Confirm  [Esc]Cancel  [Backspace]Delete char",
        };

        frame.render_widget(Paragraph::new(hint).centered().fg(Color::DarkGray), hint_area);

        if let Some(input_rect) = input_area {
            if let Some(input_area_widget) = &self.input_widget {
                frame.render_widget(input_area_widget, input_rect);
                input_area_widget.render_calendar(frame, area);
            }
        }
    }
}
