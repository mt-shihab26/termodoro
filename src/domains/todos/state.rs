use time::{Date, Duration, Month, OffsetDateTime};

use super::{mode::Mode, todo::{Repeat, Todo}};

pub struct TodosState {
    pub items: Vec<Todo>,
    pub selected: usize,
    pub mode: Mode,
    pub input: String,
    pub calendar_date: Date,
    pub calendar_view: Date,
    pub repeat_cursor: usize,
    pub pending_due: Option<Date>,
    pub editing_idx: Option<usize>,
}

impl TodosState {
    pub fn new() -> Self {
        let today = Self::today();
        Self {
            items: Todo::fakes(),
            selected: 0,
            mode: Mode::Normal,
            input: String::new(),
            calendar_date: today,
            calendar_view: today,
            repeat_cursor: 0,
            pending_due: None,
            editing_idx: None,
        }
    }

    fn today() -> Date {
        OffsetDateTime::now_local()
            .unwrap_or_else(|_| OffsetDateTime::now_utc())
            .date()
    }

    pub fn move_down(&mut self) {
        if !self.items.is_empty() {
            self.selected = (self.selected + 1).min(self.items.len() - 1);
        }
    }

    pub fn move_up(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    pub fn toggle_selected(&mut self) {
        if !self.items.is_empty() {
            self.items[self.selected].done = !self.items[self.selected].done;
        }
    }

    pub fn delete_selected(&mut self) {
        if !self.items.is_empty() {
            self.items.remove(self.selected);
            if !self.items.is_empty() {
                self.selected = self.selected.min(self.items.len() - 1);
            } else {
                self.selected = 0;
            }
        }
    }

    pub fn start_adding(&mut self) {
        self.mode = Mode::Adding;
        self.input.clear();
    }

    pub fn confirm_add(&mut self) {
        if !self.input.trim().is_empty() {
            let today = Self::today();
            self.calendar_date = today;
            self.calendar_view = today;
            self.pending_due = None;
            self.editing_idx = None;
            self.mode = Mode::SelectingDate;
        } else {
            self.mode = Mode::Normal;
            self.input.clear();
        }
    }

    pub fn cancel_add(&mut self) {
        self.mode = Mode::Normal;
        self.input.clear();
    }

    // --- Calendar navigation ---

    pub fn calendar_nav_left(&mut self) {
        if let Some(d) = self.calendar_date.previous_day() {
            self.calendar_date = d;
            self.calendar_view = d;
        }
    }

    pub fn calendar_nav_right(&mut self) {
        if let Some(d) = self.calendar_date.next_day() {
            self.calendar_date = d;
            self.calendar_view = d;
        }
    }

    pub fn calendar_nav_up(&mut self) {
        if let Some(d) = self.calendar_date.checked_sub(Duration::weeks(1)) {
            self.calendar_date = d;
            self.calendar_view = d;
        }
    }

    pub fn calendar_nav_down(&mut self) {
        if let Some(d) = self.calendar_date.checked_add(Duration::weeks(1)) {
            self.calendar_date = d;
            self.calendar_view = d;
        }
    }

    pub fn calendar_prev_month(&mut self) {
        let month_num = self.calendar_date.month() as u8;
        let year = self.calendar_date.year();
        let (new_year, new_month_num) = if month_num == 1 {
            (year - 1, 12u8)
        } else {
            (year, month_num - 1)
        };
        if let Ok(new_month) = Month::try_from(new_month_num) {
            let new_day = self.calendar_date.day().min(days_in_month(new_year, new_month));
            if let Ok(d) = Date::from_calendar_date(new_year, new_month, new_day) {
                self.calendar_date = d;
                self.calendar_view = d;
            }
        }
    }

    pub fn calendar_next_month(&mut self) {
        let month_num = self.calendar_date.month() as u8;
        let year = self.calendar_date.year();
        let (new_year, new_month_num) = if month_num == 12 {
            (year + 1, 1u8)
        } else {
            (year, month_num + 1)
        };
        if let Ok(new_month) = Month::try_from(new_month_num) {
            let new_day = self.calendar_date.day().min(days_in_month(new_year, new_month));
            if let Ok(d) = Date::from_calendar_date(new_year, new_month, new_day) {
                self.calendar_date = d;
                self.calendar_view = d;
            }
        }
    }

    pub fn set_date_today(&mut self) {
        let today = Self::today();
        self.calendar_date = today;
        self.calendar_view = today;
    }

    pub fn set_date_yesterday(&mut self) {
        let today = Self::today();
        if let Some(d) = today.previous_day() {
            self.calendar_date = d;
            self.calendar_view = d;
        }
    }

    pub fn set_date_tomorrow(&mut self) {
        let today = Self::today();
        if let Some(d) = today.next_day() {
            self.calendar_date = d;
            self.calendar_view = d;
        }
    }

    /// Confirm the selected date and create/update the todo without a repeat.
    pub fn confirm_date(&mut self) {
        self.pending_due = Some(self.calendar_date);
        self.apply_todo(None);
    }

    /// Open the repeat section below the calendar (switches to SelectingRepeat).
    pub fn open_repeat(&mut self) {
        self.pending_due = Some(self.calendar_date);
        self.repeat_cursor = 0;
        self.mode = Mode::SelectingRepeat;
    }

    /// Cancel date selection entirely — discards the in-progress add/edit.
    pub fn cancel_selecting_date(&mut self) {
        if self.editing_idx.is_none() {
            self.input.clear();
        }
        self.editing_idx = None;
        self.pending_due = None;
        self.mode = Mode::Normal;
    }

    // --- Repeat selection ---

    pub fn repeat_move_up(&mut self) {
        self.repeat_cursor = self.repeat_cursor.saturating_sub(1);
    }

    pub fn repeat_move_down(&mut self) {
        self.repeat_cursor = (self.repeat_cursor + 1).min(5);
    }

    /// Confirm the highlighted repeat option and create/update the todo.
    pub fn confirm_repeat(&mut self) {
        let repeat = repeat_from_cursor(self.repeat_cursor);
        self.apply_todo(repeat);
    }

    /// Go back to the calendar without committing the todo yet.
    pub fn cancel_repeat(&mut self) {
        self.mode = Mode::SelectingDate;
    }

    fn apply_todo(&mut self, repeat: Option<Repeat>) {
        if let Some(idx) = self.editing_idx {
            self.items[idx].due_date = self.pending_due;
            self.items[idx].repeat = repeat;
            self.editing_idx = None;
        } else {
            let text = self.input.trim().to_string();
            let mut todo = Todo::new(&text);
            todo.due_date = self.pending_due;
            todo.repeat = repeat;
            self.items.push(todo);
            self.selected = self.items.len() - 1;
            self.input.clear();
        }
        self.pending_due = None;
        self.mode = Mode::Normal;
    }

    pub fn start_edit_date(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let idx = self.selected;
        let date = self.items[idx].due_date.unwrap_or_else(Self::today);
        self.calendar_date = date;
        self.calendar_view = date;
        self.pending_due = self.items[idx].due_date;
        self.editing_idx = Some(idx);
        self.mode = Mode::SelectingDate;
    }
}

fn repeat_from_cursor(cursor: usize) -> Option<Repeat> {
    match cursor {
        1 => Some(Repeat::Daily),
        2 => Some(Repeat::WeeklySameDay),
        3 => Some(Repeat::WeekdaysMonFri),
        4 => Some(Repeat::MonthlyOnDay),
        5 => Some(Repeat::YearlyOnDay),
        _ => None,
    }
}

fn days_in_month(year: i32, month: Month) -> u8 {
    let (next_year, next_month_num) = if month == Month::December {
        (year + 1, 1u8)
    } else {
        (year, month as u8 + 1)
    };
    if let Ok(next_month) = Month::try_from(next_month_num) {
        if let Ok(first_of_next) = Date::from_calendar_date(next_year, next_month, 1) {
            if let Some(last) = first_of_next.previous_day() {
                return last.day();
            }
        }
    }
    28
}
