use crate::Time;
use chrono::NaiveDateTime;

impl Time {
    pub fn resolve(self, now: NaiveDateTime) -> NaiveDateTime {
        match self {
            Self::Absolute(time) => NaiveDateTime::new(now.date(), time),
            Self::Relative(time) => time.resolve(now),
        }
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

    mod given_absolute_time {
        use super::*;

        #[test]
        fn returns_given_datetime_with_modified_time() {
            let actual = Time::Absolute(NaiveTime::from_hms(12, 34, 56)).resolve(now());
            let expected = NaiveDateTime::new(now().date(), NaiveTime::from_hms(12, 34, 56));

            assert_eq!(expected, actual);
        }
    }

    mod given_relative_time {
        use super::*;
        use crate::RelativeTime;

        #[test]
        fn returns_given_datetime_plus_that_relative_time() {
            let actual = Time::Relative(RelativeTime {
                hours: Some(1),
                minutes: Some(2),
                seconds: Some(3),
            })
            .resolve(now());

            let expected = NaiveDateTime::new(now().date(), NaiveTime::from_hms(02, 25, 48));

            assert_eq!(expected, actual);
        }
    }
}
