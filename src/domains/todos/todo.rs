use time::Date;

use super::repeat::Repeat;

pub struct Todo {
    pub text: String,
    pub done: bool,
    pub due_date: Option<Date>,
    pub repeat: Option<Repeat>,
    pub completions: Vec<Date>,
}

impl Todo {
    pub fn new(text: String, due_date: Option<Date>, repeat: Option<Repeat>) -> Self {
        Self {
            text,
            done: false,
            due_date,
            repeat,
            completions: vec![],
        }
    }

    pub fn toggle(&mut self) {
        if self.done {
            self.done = false;
        } else {
            if let Some(date) = self.due_date {
                self.completions.push(date);
            }
            if let (Some(repeat), Some(date)) = (self.repeat, self.due_date) {
                self.due_date = Some(repeat.next_date(date));
            } else {
                self.done = true;
            }
        }
    }

    pub fn fakes() -> Vec<Todo> {
        use time::OffsetDateTime;

        let today = OffsetDateTime::now_utc().date();
        let day = time::Duration::days(1);

        let mut todos = vec![
            // -5 days
            Todo::new("Submit quarterly report".to_string(), Some(today - day * 5), None),
            // -4 days
            Todo::new(
                "Team retrospective meeting".to_string(),
                Some(today - day * 4),
                Some(super::repeat::Repeat::WeeklySameDay),
            ),
            // -3 days
            Todo::new(
                "Review pull requests".to_string(),
                Some(today - day * 3),
                Some(super::repeat::Repeat::WeekdaysMonFri),
            ),
            // -2 days
            Todo::new("Update project roadmap".to_string(), Some(today - day * 2), None),
            // -1 day (yesterday)
            Todo::new("Call the dentist".to_string(), Some(today - day), None),
            // today
            Todo::new("Nothing".to_string(), Some(today), None),
            Todo::new(
                "Pay utility bills".to_string(),
                Some(today),
                Some(super::repeat::Repeat::MonthlyOnDay),
            ),
            Todo::new(
                "Morning standup".to_string(),
                Some(today),
                Some(super::repeat::Repeat::WeekdaysMonFri),
            ),
            // +1 day
            Todo::new("Grocery shopping".to_string(), Some(today + day), None),
            // +2 days
            Todo::new("Doctor appointment".to_string(), Some(today + day * 2), None),
            // +3 days
            Todo::new(
                "Deploy new release".to_string(),
                Some(today + day * 3),
                Some(super::repeat::Repeat::Daily),
            ),
            // +4 days
            Todo::new("Book flight tickets".to_string(), Some(today + day * 4), None),
            // +5 days
            Todo::new(
                "Annual performance review".to_string(),
                Some(today + day * 5),
                Some(super::repeat::Repeat::YearlyOnDay),
            ),
        ];

        // seed some fake history
        todos[0].completions = vec![today - day * 12, today - day * 5];
        todos[0].done = true;
        todos[1].completions = vec![today - day * 11, today - day * 4];
        todos[2].completions = vec![today - day * 10, today - day * 3];
        todos[3].completions = vec![today - day * 9];
        todos[4].completions = vec![today - day * 8];

        todos
    }
}
