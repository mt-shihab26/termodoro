use chrono::Local;

pub fn today_string() -> String {
    Local::now().date_naive().format("%Y-%m-%d").to_string()
}

pub fn unix_now() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

pub fn parse_todo_input(input: &str) -> (String, Option<String>) {
    // Convention: if the last token looks like a date, treat it as due_date.
    let trimmed = input.trim();
    if let Some((head, tail)) = trimmed.rsplit_once(' ') {
        if looks_like_date(tail) {
            return (head.trim().to_string(), Some(tail.to_string()));
        }
        if let Ok(Some(d)) = parse_due_date_input(tail) {
            return (head.trim().to_string(), Some(d));
        }
    }
    (trimmed.to_string(), None)
}

pub fn parse_due_date_input(input: &str) -> Result<Option<String>, String> {
    let t = input.trim();
    if t.is_empty() {
        return Ok(None);
    }
    if t.eq_ignore_ascii_case("none") || t.eq_ignore_ascii_case("clear") {
        return Ok(None);
    }
    if t.eq_ignore_ascii_case("today") {
        return Ok(Some(today_string()));
    }
    if t.eq_ignore_ascii_case("tomorrow") {
        let d = Local::now().date_naive() + chrono::Duration::days(1);
        return Ok(Some(d.format("%Y-%m-%d").to_string()));
    }
    if looks_like_date(t) {
        return Ok(Some(t.to_string()));
    }
    Err("due date: expected YYYY-MM-DD, today, tomorrow, or empty".to_string())
}

pub fn looks_like_date(s: &str) -> bool {
    let b = s.as_bytes();
    if b.len() != 10 {
        return false;
    }
    b[0..4].iter().all(|c| c.is_ascii_digit())
        && b[4] == b'-'
        && b[5..7].iter().all(|c| c.is_ascii_digit())
        && b[7] == b'-'
        && b[8..10].iter().all(|c| c.is_ascii_digit())
}

pub fn format_work(secs: u64) -> String {
    let mins = secs / 60;
    let h = mins / 60;
    let m = mins % 60;
    if h > 0 { format!("{h}h{m:02}m") } else { format!("{m}m") }
}
