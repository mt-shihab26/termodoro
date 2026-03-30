#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Repeat {
    Daily,
    WeeklySameDay,
    WeekdaysMonFri,
    MonthlyOnDay,
    YearlyOnDay,
}

impl Repeat {
    pub const ALL: &'static [Repeat] = &[
        Repeat::Daily,
        Repeat::WeeklySameDay,
        Repeat::WeekdaysMonFri,
        Repeat::MonthlyOnDay,
        Repeat::YearlyOnDay,
    ];

    pub fn icon() -> &'static str {
        "⟳"
    }

    pub fn label(&self) -> &str {
        match self {
            Repeat::Daily => "Daily",
            Repeat::WeeklySameDay => "Weekly (same day)",
            Repeat::WeekdaysMonFri => "Weekdays (Mon-Fri)",
            Repeat::MonthlyOnDay => "Monthly on day",
            Repeat::YearlyOnDay => "Yearly on day",
        }
    }

    pub fn next_date(&self, from: time::Date) -> time::Date {
        match self {
            Repeat::Daily => from + time::Duration::days(1),
            Repeat::WeeklySameDay => from + time::Duration::weeks(1),
            Repeat::WeekdaysMonFri => {
                let mut next = from + time::Duration::days(1);
                while matches!(next.weekday(), time::Weekday::Saturday | time::Weekday::Sunday) {
                    next = next + time::Duration::days(1);
                }
                next
            }
            Repeat::MonthlyOnDay => {
                let month = from.month().next();
                let year = if month == time::Month::January { from.year() + 1 } else { from.year() };
                time::Date::from_calendar_date(year, month, from.day()).unwrap_or(from)
            }
            Repeat::YearlyOnDay => {
                time::Date::from_calendar_date(from.year() + 1, from.month(), from.day()).unwrap_or(from)
            }
        }
    }
}
