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

    pub fn label(&self) -> &str {
        match self {
            Repeat::Daily => "Daily",
            Repeat::WeeklySameDay => "Weekly (same day)",
            Repeat::WeekdaysMonFri => "Weekdays (Mon-Fri)",
            Repeat::MonthlyOnDay => "Monthly on day",
            Repeat::YearlyOnDay => "Yearly on day",
        }
    }
}
