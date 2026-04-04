use std::{
    cell::Ref,
    io::Result,
    sync::{Arc, Mutex},
};

use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    prelude::{Color, Constraint, Layout, Rect, Stylize, Widget},
    widgets::Block,
};
use sea_orm::DatabaseConnection;

use crate::{
    caches::timer::TimerCache,
    kinds::{page::Page, todos_mode::TodosMode},
    models::todo::Todo,
    states::todos::TodosState,
    utils::date::today,
    widgets::todos::{
        hint::{HintProps, HintWidget},
        input::{InputAction, InputProps, InputState, InputWidget},
        list::{ListProps, ListWidget},
        status::{StatusProps, StatusWidget},
        tabs::{TabsProps, TabsWidget},
    },
};

use super::Tab;

pub const COLOR: Color = Color::Green;

pub struct TodosTab {
    page: Page,
    mode: TodosMode,
    state: TodosState,
    input_state: Option<InputState>,
}

impl TodosTab {
    pub fn new(db: DatabaseConnection, timer_cache: Arc<Mutex<TimerCache>>) -> Self {
        Self {
            page: Page::Today,
            mode: TodosMode::Normal,
            state: TodosState::new(db, timer_cache),
            input_state: None,
        }
    }

    fn items(&self) -> Ref<'_, [Todo]> {
        self.state.items(self.page)
    }

    fn count(&self) -> usize {
        self.state.count(self.page)
    }

    fn set_page(&mut self, page: Page) {
        self.page = page;
        self.state.reset_page(self.page);
    }

    fn cancel_input(&mut self) {
        self.input_state = None;
        self.mode = TodosMode::Normal;
    }
}

impl Tab for TodosTab {
    fn name(&self) -> &str {
        "Todos [^t]"
    }

    fn color(&self) -> Color {
        COLOR
    }

    fn handle(&mut self, key: KeyEvent) -> Result<()> {
        let pending_g = self.state.begin_input();

        match self.mode {
            TodosMode::Normal => match key.code {
                KeyCode::Char('1') => self.set_page(Page::Due),
                KeyCode::Char('2') => self.set_page(Page::Today),
                KeyCode::Char('3') => self.set_page(Page::Index),
                KeyCode::Char('4') => self.set_page(Page::History),
                KeyCode::Char(']') => self.set_page(self.page.next()),
                KeyCode::Char('[') => self.set_page(self.page.prev()),
                KeyCode::Char('j') | KeyCode::Down => self.state.move_selection(self.page, 1),
                KeyCode::Char('k') | KeyCode::Up => self.state.move_selection(self.page, -1),
                KeyCode::Char('g') => self.state.go_to_start(pending_g),
                KeyCode::Char('G') => self.state.go_to_end(self.page),
                KeyCode::Char(' ') | KeyCode::Enter => self.state.toggle_selected(self.page),
                KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.state.delete_selected(self.page)
                }
                KeyCode::Char('a') => {
                    self.mode = TodosMode::Adding;
                    let date = if self.page == Page::Today { Some(today()) } else { None };
                    self.input_state = Some(InputState::new(InputProps::new(None, date, None)));
                }
                KeyCode::Char('e') => {
                    if let Some((text, due_date, repeat)) = self.state.edit_values(self.page) {
                        self.mode = TodosMode::Editing;
                        self.input_state =
                            Some(InputState::new(InputProps::new(Some(&text), due_date, repeat.as_ref())));
                    }
                }
                _ => {}
            },
            TodosMode::Adding => {
                if let Some(input_widget) = &mut self.input_state {
                    match input_widget.handle(key) {
                        InputAction::Confirm { text, date, repeat } => {
                            self.state.add(self.page, text, date, repeat);
                            self.cancel_input();
                        }
                        InputAction::Escape => self.cancel_input(),
                        InputAction::None => {}
                    }
                }
            }
            TodosMode::Editing => {
                if let Some(input_widget) = &mut self.input_state {
                    match input_widget.handle(key) {
                        InputAction::Confirm { text, date, repeat } => {
                            self.state.update(self.page, text, date, repeat);
                            self.cancel_input();
                        }
                        InputAction::Escape => self.cancel_input(),
                        InputAction::None => {}
                    }
                }
            }
        }
        self.state.sync_list_state(self.items().len());
        Ok(())
    }

    fn render(&self, frame: &mut Frame, area: Rect) {
        let buf = frame.buffer_mut();

        let block = Block::bordered().fg(self.color());
        let inner = block.inner(area);
        block.render(area, buf);

        let area = inner;

        let (tabs_area, list_area, hint_area, input_area) = match self.mode {
            TodosMode::Normal => {
                let [tabs, list, hint] =
                    Layout::vertical([Constraint::Length(1), Constraint::Fill(1), Constraint::Length(1)]).areas(area);
                (tabs, list, hint, None)
            }
            TodosMode::Adding | TodosMode::Editing => {
                let [tabs, list, hint, input] = Layout::vertical([
                    Constraint::Length(1),
                    Constraint::Fill(1),
                    Constraint::Length(1),
                    Constraint::Length(3),
                ])
                .areas(area);
                (tabs, list, hint, Some(input))
            }
        };

        self.state.set_visible_capacity(list_area);
        let items = self.items();
        let stats = self.state.stats(self.page);
        let total = self.count();
        let from = self.state.from(total);
        let to = self.state.to(items.len());
        let page = self.state.page();

        frame.render_widget(&StatusWidget::new(&StatusProps::new(total, from, to, page)), area);

        frame.render_widget(&TabsWidget::new(&TabsProps::new(self.page, self.color())), tabs_area);

        frame.render_widget(
            &ListWidget::new(&ListProps::new(
                &items,
                &stats,
                self.state.offset(),
                self.page,
                self.state.selected(),
                self.color(),
                self.state.show_more_above(),
                self.state.show_more_below(items.len()),
            )),
            list_area,
        );

        HintWidget::new(&HintProps::new(self.mode, self.state.can_delete(self.page, &items)))
            .render(hint_area, frame.buffer_mut());

        if let Some(input_rect) = input_area {
            if let Some(input_state) = &self.input_state {
                let props = input_state.props();
                frame.render_widget(&InputWidget::new(&props), input_rect);
                input_state.render_calendar(frame, area);
            }
        }
    }
}
