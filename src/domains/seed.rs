use time::{Date, Duration};

use crate::{kinds::repeat::Repeat, models::todo::Todo, utils::date::today};

pub fn seed_todos(count: usize) -> Vec<Todo> {
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

    let mut todos: Vec<Todo> = (0..count)
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

            Todo::new(text, due_date, repeat, None)
        })
        .collect();

    todos.extend(focused_examples(base));
    todos
}

fn focused_examples(base: Date) -> Vec<Todo> {
    vec![
        Todo::new("Today: pay electricity bill".to_string(), Some(base), None, None),
        Todo::new(
            "Today: call mom".to_string(),
            Some(base),
            Some(Repeat::WeeklySameDay),
            None,
        ),
        Todo::new(
            "Overdue: renew passport".to_string(),
            Some(base - Duration::days(2)),
            None,
            None,
        ),
        Todo::new(
            "Upcoming: draft Q2 plan".to_string(),
            Some(base + Duration::days(3)),
            None,
            None,
        ),
        Todo::new(
            "Upcoming: yearly health check".to_string(),
            Some(base + Duration::days(30)),
            Some(Repeat::YearlyOnDay),
            None,
        ),
        Todo::new("No date: reorganize bookshelf".to_string(), None, None, None),
    ]
}
