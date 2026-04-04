use time::{Date, Duration};

use crate::{kinds::repeat::Repeat, utils::date::today};

pub struct SeedTodo {
    pub text: String,
    pub due_date: Option<Date>,
    pub repeat: Option<Repeat>,
}

impl SeedTodo {
    fn new(text: &str, due_date: Option<Date>, repeat: Option<Repeat>) -> SeedTodo {
        SeedTodo {
            text: text.to_string(),
            due_date,
            repeat,
        }
    }
}

pub fn seed_todos(count: usize) -> Vec<SeedTodo> {
    let base = today();
    let projects = [
        "Inbox",
        "Home",
        "Health",
        "Study",
        "Reading",
        "Work",
        "Finance",
        "Errands",
        "Writing",
        "Deep Work",
    ];
    let actions = [
        "review notes",
        "clean desk",
        "send update",
        "write summary",
        "fix bug",
        "call dentist",
        "plan sprint",
        "buy groceries",
        "archive receipts",
        "practice rust",
    ];

    let mut todos: Vec<SeedTodo> = (0..count)
        .map(|i| {
            let text = format!(
                "{} {} {}",
                projects[i % projects.len()],
                i + 1,
                actions[(i * 7) % actions.len()]
            );

            let due_date = match i % 6 {
                0 => Some(base - Duration::days((i % 9 + 1) as i64)),
                1 => Some(base),
                2 | 3 => Some(base + Duration::days((i % 14 + 1) as i64)),
                4 => Some(base + Duration::days((i % 45 + 15) as i64)),
                _ => None,
            };

            let repeat = match i % 10 {
                0 => Some(Repeat::Daily),
                1 => Some(Repeat::WeeklySameDay),
                2 => Some(Repeat::WeekdaysMonFri),
                3 => Some(Repeat::MonthlyOnDay),
                _ => None,
            };

            SeedTodo { text, due_date, repeat }
        })
        .collect();

    todos.extend(focused_examples(base));
    todos
}

fn focused_examples(base: Date) -> Vec<SeedTodo> {
    vec![
        SeedTodo::new("Today: pay electricity bill", Some(base), None),
        SeedTodo::new("Today: call mom", Some(base), Some(Repeat::WeeklySameDay)),
        SeedTodo::new("Overdue: renew passport", Some(base - Duration::days(2)), None),
        SeedTodo::new("Upcoming: draft Q2 plan", Some(base + Duration::days(3)), None),
        SeedTodo::new(
            "Upcoming: yearly health check",
            Some(base + Duration::days(30)),
            Some(Repeat::YearlyOnDay),
        ),
        SeedTodo::new("No date: reorganize bookshelf", None, None),
    ]
}
