use chrono::Utc;
use serde::Serialize;

mod parse;
mod resolve;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum DateTime {
    Today { hour: usize, minute: usize },
    Tomorrow { hour: usize, minute: usize },
    NextDayOfWeek { day_of_week: DayOfWeek },
    Arbitrary(chrono::DateTime<Utc>),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
