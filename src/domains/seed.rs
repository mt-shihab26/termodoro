use time::{Date, Duration};

use crate::kinds::repeat::Repeat;
use crate::models::todo::Todo;
use crate::utils::date::today;

pub fn seed_todos() -> Vec<Todo> {
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

    let mut todos = Vec::new();

    for i in 0..1000 {
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

        let mut todo = Todo::new(text, due_date, repeat);
        if i % 8 == 0 {
            todo.done = true;
        }
        todos.push(todo);
    }

    todos.extend(focused_examples(base));
    todos
}

fn focused_examples(base: Date) -> Vec<Todo> {
    vec![
        make("Today: pay electricity bill", Some(base), None, false),
        make("Today: call mom", Some(base), Some(Repeat::WeeklySameDay), false),
        make("Overdue: renew passport", Some(base - Duration::days(2)), None, false),
        make("Upcoming: draft Q2 plan", Some(base + Duration::days(3)), None, false),
        make(
            "Upcoming: yearly health check",
            Some(base + Duration::days(30)),
            Some(Repeat::YearlyOnDay),
            false,
        ),
        make("No date: reorganize bookshelf", None, None, false),
        make("Done today: clear inbox", Some(base), None, true),
        make("Done no date: migrate notes", None, None, true),
    ]
}

fn make(text: &str, due_date: Option<Date>, repeat: Option<Repeat>, done: bool) -> Todo {
    let mut todo = Todo::new(text.to_string(), due_date, repeat);
    todo.done = done;
    todo
}
