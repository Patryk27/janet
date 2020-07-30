use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum DateTimeSpec {
    Today { hour: usize, minute: usize },
    Tomorrow { hour: usize, minute: usize },
    NextDayOfWeek { day_of_week: DateTimeSpecDayOfWeek },
    Arbitrary(DateTime<Utc>),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum DateTimeSpecDayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl DateTimeSpec {
    pub fn to_absolute(self, now: DateTime<Utc>) -> DateTime<Utc> {
        todo!()
    }
}
