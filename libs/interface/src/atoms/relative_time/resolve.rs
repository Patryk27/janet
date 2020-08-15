use crate::RelativeTime;
use chrono::{Duration, NaiveDateTime};

impl RelativeTime {
    pub fn resolve(self, now: NaiveDateTime) -> NaiveDateTime {
        now + Duration::hours(self.hours.unwrap_or_default() as i64)
            + Duration::minutes(self.minutes.unwrap_or_default() as i64)
            + Duration::seconds(self.seconds.unwrap_or_default() as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveTime};

    fn now() -> NaiveDateTime {
        NaiveDateTime::new(
            NaiveDate::from_ymd(2012, 01, 01),
            NaiveTime::from_hms(01, 23, 45),
        )
    }

    #[test]
    fn returns_given_datetime_plus_duration() {
        let actual = RelativeTime {
            hours: Some(1),
            minutes: Some(2),
            seconds: Some(3),
        }
        .resolve(now());

        let expected = NaiveDateTime::new(now().date(), NaiveTime::from_hms(02, 25, 48));

        assert_eq!(expected, actual);
    }
}
