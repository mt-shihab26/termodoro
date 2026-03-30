use time::Date;

use super::repeat::Repeat;

pub struct Todo {
    pub text: String,
    pub done: bool,
    pub due_date: Option<Date>,
    pub repeat: Option<Repeat>,
}

impl Todo {
    pub fn new(text: String, due_date: Option<Date>, repeat: Option<Repeat>) -> Self {
        Self {
            text,
            done: false,
            due_date,
            repeat,
        }
    }

    pub fn fakes() -> Vec<Todo> {
        use time::OffsetDateTime;

        let today = OffsetDateTime::now_utc().date();
        let day = time::Duration::days(1);

        vec![
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
        ]
    }
}
