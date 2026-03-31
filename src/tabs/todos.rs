use std::cell::Ref;
use std::io::Result;

use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Block, Widget};
use sea_orm::DatabaseConnection;

use crate::kinds::mode::Mode;
use crate::kinds::{page::Page, repeat::Repeat};
use crate::models::todo::Todo;
use crate::states::todos::TodosState;
use crate::widgets::todos::hint::HintWidget;
use crate::widgets::todos::input::{InputAction, InputWidget};
use crate::widgets::todos::list::ListWidget;
use crate::widgets::todos::status::StatusWidget;
use crate::widgets::todos::tabs::TabsWidget;

use super::Tab;

pub const COLOR: Color = Color::Green;

pub struct Todos {
    db: DatabaseConnection,
    page: Page,
    mode: Mode,
    state: TodosState,
    input_widget: Option<InputWidget>,
}

impl Tab for Todos {
    fn name(&self) -> &str {
        "Todos [^t]"
    }

    fn color(&self) -> Color {
        COLOR
    }

    fn handle(&mut self, key: KeyEvent) -> Result<()> {
        let pending_g = self.state.begin_input();

        match self.mode {
            Mode::Normal => match key.code {
                KeyCode::Char('1') => self.set_page(Page::Due),
                KeyCode::Char('2') => self.set_page(Page::Today),
                KeyCode::Char('3') => self.set_page(Page::Index),
                KeyCode::Char('4') => self.set_page(Page::History),
                KeyCode::Char(']') => self.set_page(self.page.next()),
                KeyCode::Char('[') => self.set_page(self.page.prev()),
                KeyCode::Char('j') | KeyCode::Down => self.move_selection(1),
                KeyCode::Char('k') | KeyCode::Up => self.move_selection(-1),
                KeyCode::Char('g') => self.go_to_start(pending_g),
                KeyCode::Char('G') => self.go_to_end(),
                KeyCode::Char(' ') | KeyCode::Enter => {
                    if let Some(mut todo) = self.selected_item().map(|todo| todo.clone()) {
                        todo.toggle(&self.db);
                        self.refresh();
                    }
                }
                KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    if self.can_delete_selected() {
                        if let Some(should_delete) = self.selected_item().map(|todo| {
                            if todo.done {
                                false
                            } else {
                                todo.delete(&self.db)
                            }
                        }) {
                            if should_delete {
                                self.refresh();
                            }
                        }
                    }
                }
                KeyCode::Char('a') => {
                    if !matches!(self.page, Page::History) {
                        self.mode = Mode::Adding;
                        self.input_widget = Some(InputWidget::new(None, None, None));
                    }
                }
                KeyCode::Char('e') => {
                    if !matches!(self.page, Page::History) {
                        if let Some((text, due_date, repeat)) = self
                            .selected_item()
                            .map(|todo| (todo.text.clone(), todo.due_date, todo.repeat.clone()))
                        {
                            self.mode = Mode::Editing;
                            self.input_widget =
                                Some(InputWidget::new(Some(&text), due_date, repeat.as_ref()));
                        }
                    }
                }
                _ => {}
            },
            Mode::Adding => {
                if let Some(input_widget) = &mut self.input_widget {
                    match input_widget.handle(key) {
                        InputAction::Confirm { text, date, repeat } => {
                            self.confirm_add(text, date, repeat)
                        }
                        InputAction::Escape => self.cancel_input(),
                        InputAction::None => {}
                    }
                }
            }
            Mode::Editing => {
                if let Some(input_widget) = &mut self.input_widget {
                    match input_widget.handle(key) {
                        InputAction::Confirm { text, date, repeat } => {
                            self.confirm_edit(text, date, repeat)
                        }
                        InputAction::Escape => self.cancel_input(),
                        InputAction::None => {}
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

        let (tabs_area, list_area, hint_area, input_area) = match self.mode {
            Mode::Normal => {
                let [tabs, list, hint] = Layout::vertical([
                    Constraint::Length(1),
                    Constraint::Fill(1),
                    Constraint::Length(1),
                ])
                .areas(area);
                (tabs, list, hint, None)
            }
            Mode::Adding | Mode::Editing => {
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
        let total = self.count();
        let from = self.state.from(total);
        let to = self.state.to(items.len());
        let page = self.state.page();

        frame.render_widget(
            StatusWidget::new(
                total,
                from,
                to,
                page,
                items.get(self.state.selected()).and_then(|todo| todo.id),
            ),
            area,
        );

        frame.render_widget(
            TabsWidget {
                page: self.page,
                color: self.color(),
            },
            tabs_area,
        );

        ListWidget {
            items: &items,
            offset: self.state.offset(),
            page: self.page,
            selected: self.state.selected(),
            color: self.color(),
            show_more_above: self.state.show_more_above(),
            show_more_below: self.state.show_more_below(items.len()),
        }
        .render(frame, list_area, &mut self.state.list_state_mut());

        frame.render_widget(
            HintWidget {
                page: self.page,
                ui_mode: self.mode,
                can_delete: self.can_delete_in_items(&items),
            },
            hint_area,
        );

        if let Some(input_rect) = input_area {
            if let Some(input_area_widget) = &self.input_widget {
                frame.render_widget(input_area_widget, input_rect);
                input_area_widget.render_calendar(frame, area);
            }
        }
    }

    fn should_tick(&self) -> bool {
        self.state.should_tick()
    }

    fn next_tick(&mut self) -> Result<()> {
        if !matches!(self.mode, Mode::Normal) {
            return Ok(());
        }

        if self.state.is_animating() {
            let changed = self.step_animation();
            if !changed {
                self.state.stop_animation();
            }
            self.sync_list_state();
        }

        Ok(())
    }
}

impl Todos {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db,
            page: Page::Today,
            mode: Mode::Normal,
            state: TodosState::new(),
            input_widget: None,
        }
    }

    fn items(&self) -> Ref<'_, [Todo]> {
        self.state.items(&self.db, self.page)
    }

    fn count(&self) -> usize {
        self.state.count(&self.db, self.page)
    }

    fn invalidate_cache(&self) {
        self.state.invalidate_items();
    }

    fn invalidate_total_count_cache(&self) {
        self.state.invalidate_count();
    }

    fn refresh(&mut self) {
        self.invalidate_cache();
        self.invalidate_total_count_cache();
        self.clamp_selected();
    }

    fn selected_item(&self) -> Option<Ref<'_, Todo>> {
        self.state.selected_item(&self.db, self.page)
    }

    fn can_delete_selected(&self) -> bool {
        let items = self.items();
        self.can_delete_in_items(&items)
    }

    fn can_delete_in_items(&self, items: &[Todo]) -> bool {
        self.state.can_delete(self.page, items)
    }

    fn clamp_selected(&mut self) {
        self.state.clamp_selected(&self.db, self.page);
    }

    fn sync_list_state(&self) {
        self.state.sync_list_state(self.items().len());
    }

    fn set_page(&mut self, page: Page) {
        self.page = page;
        self.state.reset_page();
        self.invalidate_cache();
        self.invalidate_total_count_cache();
    }

    fn go_to_start(&mut self, pending_g: bool) {
        self.state.go_to_start(pending_g);
    }

    fn go_to_end(&mut self) {
        self.state.go_to_end();
    }

    fn move_selection(&mut self, delta: isize) {
        self.state.move_selection(&self.db, self.page, delta);
    }

    fn step_animation(&mut self) -> bool {
        self.state.step_animation(&self.db, self.page)
    }

    fn cancel_input(&mut self) {
        self.input_widget = None;
        self.mode = Mode::Normal;
    }

    fn confirm_add(&mut self, text: String, date: Option<time::Date>, repeat: Option<Repeat>) {
        let mut todo = Todo::new(text, date, repeat);
        if todo.save(&self.db) {
            self.refresh();
        }
        self.cancel_input();
    }

    fn confirm_edit(&mut self, text: String, date: Option<time::Date>, repeat: Option<Repeat>) {
        if let Some(mut todo) = self.selected_item().map(|todo| todo.clone()) {
            todo.text = text;
            todo.due_date = date;
            todo.repeat = repeat;
            todo.update(&self.db);
            self.refresh();
        }
        self.cancel_input();
    }
}
