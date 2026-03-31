use time::{Date, Duration};

#[derive(Debug, PartialEq)]
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

    pub fn next_date(&self, from: Date) -> Date {
        match self {
            Repeat::Daily => from + Duration::days(1),
            Repeat::WeeklySameDay => from + Duration::weeks(1),
            Repeat::WeekdaysMonFri => {
                let mut next = from + Duration::days(1);
                while matches!(next.weekday(), time::Weekday::Saturday | time::Weekday::Sunday) {
                    next = next + Duration::days(1);
                }
                next
            }
            Repeat::MonthlyOnDay => {
                let month = from.month().next();
                let year = if month == time::Month::January {
                    from.year() + 1
                } else {
                    from.year()
                };
                Date::from_calendar_date(year, month, from.day()).unwrap_or(from)
            }
            Repeat::YearlyOnDay => Date::from_calendar_date(from.year() + 1, from.month(), from.day()).unwrap_or(from),
        }
    }
}
