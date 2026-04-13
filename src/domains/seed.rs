use sea_orm::DatabaseConnection;
use time::{Duration, OffsetDateTime};

use crate::{kinds::repeat::Repeat, models::todo::Todo, utils::date::now};

/// Seeds the database with fake todos and returns how many were inserted.
pub fn seed_todos(count: usize, db: &DatabaseConnection) -> usize {
    let base = now();
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

    let mut inserted = 0usize;

    // Save the main batch; collect repeating ones so we can seed children with their IDs.
    let mut repeating: Vec<Todo> = Vec::new();

    for i in 0..count {
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
            1 => Some(Repeat::Weekly),
            2 => Some(Repeat::Weekends),
            3 => Some(Repeat::Weekdays),
            4 => Some(Repeat::Monthly),
            _ => None,
        };

        let is_repeating = repeat.is_some();
        let mut todo = Todo::new(text, due_date, repeat, None);
        todo.done_at = if i % 3 != 0 { Some(now()) } else { None };
        if todo.save(db) {
            inserted += 1;
            if is_repeating {
                repeating.push(todo);
            }
        }
    }

    // Seed a past occurrence for some repeating todos to simulate the parent_id relationship.
    for parent in repeating.iter().take(5) {
        if let Some(child) = parent.save_next(db) {
            let _ = child;
            inserted += 1;
        }
    }

    inserted += seed_focused(base, db);
    inserted
}

/// Inserts a small curated set of todos for common demo states.
fn seed_focused(base: OffsetDateTime, db: &DatabaseConnection) -> usize {
    let items = [
        Todo::new("Today: pay electricity bill".to_string(), Some(base), None, None),
        Todo::new("Today: call mom".to_string(), Some(base), Some(Repeat::Weekly), None),
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
            Some(Repeat::Yearly),
            None,
        ),
        Todo::new("No date: reorganize bookshelf".to_string(), None, None, None),
    ];

    let mut count = 0;
    for mut t in items {
        if t.save(db) {
            count += 1;
        }
    }
    count
}
