use time::{Date, Month, OffsetDateTime, PrimitiveDateTime, Time};

/// Returns the current UTC time
pub fn now_utc() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

/// Returns today's local date, falling back to UTC if the local offset is unavailable.
pub fn today() -> Date {
    OffsetDateTime::now_local()
        .unwrap_or_else(|_| OffsetDateTime::now_utc())
        .date()
}

/// Formats an `OffsetDateTime` as `"YYYY-MM-DDTHH:MM:SSZ"`.
pub fn format_datetime(dt: OffsetDateTime) -> String {
    format!(
        "{}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        dt.year(),
        dt.month() as u8,
        dt.day(),
        dt.hour(),
        dt.minute(),
        dt.second()
    )
}

/// Parses an ISO 8601 datetime string (`"YYYY-MM-DDTHH:MM:SSZ"` or `"YYYY-MM-DD"`) into an
/// `OffsetDateTime`. Date-only strings are interpreted as midnight UTC.
pub fn parse_datetime(s: &str) -> Option<OffsetDateTime> {
    if s.len() >= 19 {
        let date = parse_date(&s[..10])?;
        let hour: u8 = s[11..13].parse().ok()?;
        let minute: u8 = s[14..16].parse().ok()?;
        let second: u8 = s[17..19].parse().ok()?;
        let time = Time::from_hms(hour, minute, second).ok()?;
        Some(PrimitiveDateTime::new(date, time).assume_utc())
    } else {
        Some(PrimitiveDateTime::new(parse_date(s)?, Time::MIDNIGHT).assume_utc())
    }
}

/// Shifts a date by `delta` months, clamping the day to the last valid day of the target month.
pub fn shift_month(date: Date, delta: i32) -> Date {
    let total = date.month() as i32 - 1 + delta;
    let new_year = date.year() + total.div_euclid(12);
    let new_month_num = (total.rem_euclid(12) + 1) as u8;
    if let Ok(m) = Month::try_from(new_month_num) {
        let new_day = date.day().min(days_in_month(new_year, m));
        if let Ok(d) = Date::from_calendar_date(new_year, m, new_day) {
            return d;
        }
    }
    date
}

/// Formats a date as `"YYYY-MM-DD"`.
pub fn format_date(date: Date) -> String {
    format!("{}-{:02}-{:02}", date.year(), date.month() as u8, date.day())
}

/// Parses a `"YYYY-MM-DD"` or `"YYYY-MM-DDTHH:MM:SSZ"` string into a `Date`. Only the first 10
/// characters (the date portion) are used.
pub fn parse_date(s: &str) -> Option<Date> {
    let date_part = &s[..s.len().min(10)];
    let mut parts = date_part.splitn(3, '-');
    let year: i32 = parts.next()?.parse().ok()?;
    let month: u8 = parts.next()?.parse().ok()?;
    let day: u8 = parts.next()?.parse().ok()?;
    Date::from_calendar_date(year, Month::try_from(month).ok()?, day).ok()
}

/// Returns the number of days in the given month of the given year.
fn days_in_month(year: i32, month: Month) -> u8 {
    let (ny, nm) = if month == Month::December {
        (year + 1, 1u8)
    } else {
        (year, month as u8 + 1)
    };
    if let Ok(m) = Month::try_from(nm) {
        if let Ok(first) = Date::from_calendar_date(ny, m, 1) {
            if let Some(last) = first.previous_day() {
                return last.day();
            }
        }
    }
    28
}
