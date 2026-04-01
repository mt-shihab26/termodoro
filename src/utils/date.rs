use time::{Date, Month, OffsetDateTime};

// TODO: add comments on each fn's

pub fn now_utc_str() -> String {
    let dt = OffsetDateTime::now_utc();
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

pub fn today() -> Date {
    OffsetDateTime::now_local()
        .unwrap_or_else(|_| OffsetDateTime::now_utc())
        .date()
}

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

pub fn format_date(date: Date) -> String {
    format!("{}-{:02}-{:02}", date.year(), date.month() as u8, date.day())
}

pub fn parse_date(s: &str) -> Option<Date> {
    let mut parts = s.splitn(3, '-');
    let year: i32 = parts.next()?.parse().ok()?;
    let month: u8 = parts.next()?.parse().ok()?;
    let day: u8 = parts.next()?.parse().ok()?;
    Date::from_calendar_date(year, Month::try_from(month).ok()?, day).ok()
}

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
