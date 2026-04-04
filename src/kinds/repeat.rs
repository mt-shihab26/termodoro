use time::{Date, Duration};

/// A recurrence rule for a todo item.
#[derive(Clone, Debug, PartialEq)]
pub enum Repeat {
    /// Repeats every day.
    Daily,
    /// Repeats on the same weekday each week.
    WeeklySameDay,
    /// Repeats on weekdays only (Monday–Friday).
    WeekdaysMonFri,
    /// Repeats on the same day of each month.
    MonthlyOnDay,
    /// Repeats on the same day each year.
    YearlyOnDay,
}

impl Repeat {
    /// Returns the icon used to indicate a repeating todo.
    pub fn icon() -> &'static str {
        "⟳"
    }

    /// All repeat variants in display order.
    pub const ALL: &'static [Repeat] = &[
        Repeat::Daily,
        Repeat::WeeklySameDay,
        Repeat::WeekdaysMonFri,
        Repeat::MonthlyOnDay,
        Repeat::YearlyOnDay,
    ];

    /// Clones a `Repeat` value from a reference.
    pub fn of(r: &Repeat) -> Repeat {
        match r {
            Repeat::Daily => Repeat::Daily,
            Repeat::WeeklySameDay => Repeat::WeeklySameDay,
            Repeat::WeekdaysMonFri => Repeat::WeekdaysMonFri,
            Repeat::MonthlyOnDay => Repeat::MonthlyOnDay,
            Repeat::YearlyOnDay => Repeat::YearlyOnDay,
        }
    }

    /// Returns the human-readable label for the repeat rule.
    pub fn label(&self) -> &str {
        match self {
            Repeat::Daily => "Daily",
            Repeat::WeeklySameDay => "Weekly (same day)",
            Repeat::WeekdaysMonFri => "Weekdays (Mon-Fri)",
            Repeat::MonthlyOnDay => "Monthly on day",
            Repeat::YearlyOnDay => "Yearly on day",
        }
    }

    /// Returns the database string identifier for the repeat rule.
    pub fn to_db_str(&self) -> &str {
        match self {
            Repeat::Daily => "daily",
            Repeat::WeeklySameDay => "weekly_same_day",
            Repeat::WeekdaysMonFri => "weekdays_mon_fri",
            Repeat::MonthlyOnDay => "monthly_on_day",
            Repeat::YearlyOnDay => "yearly_on_day",
        }
    }

    /// Parses a repeat rule from its database string identifier, returning `None` if unrecognised.
    pub fn from_db_str(s: &str) -> Option<Self> {
        match s {
            "daily" => Some(Repeat::Daily),
            "weekly_same_day" => Some(Repeat::WeeklySameDay),
            "weekdays_mon_fri" => Some(Repeat::WeekdaysMonFri),
            "monthly_on_day" => Some(Repeat::MonthlyOnDay),
            "yearly_on_day" => Some(Repeat::YearlyOnDay),
            _ => None,
        }
    }

    /// Returns the next due date after `from` according to this repeat rule.
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
