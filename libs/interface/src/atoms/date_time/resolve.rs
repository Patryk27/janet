use crate::DateTime;
use anyhow::*;
use chrono::{Datelike, Local, NaiveDateTime, Timelike, Utc};

impl DateTime {
    pub fn resolve(&self, mut now: NaiveDateTime) -> NaiveDateTime {
        if let Some(date) = &self.date {
            now = NaiveDateTime::new(date.resolve(now.date()), now.time());
        }

        if let Some(time) = &self.time {
            now = time.resolve(now);
        }

        now
    }

    pub fn resolve_utc(&self, now: chrono::DateTime<Local>) -> Result<chrono::DateTime<Utc>> {
        let resolved = self.resolve(now.naive_local());

        let now: Option<_> = try {
            now.with_year(resolved.year())?
                .with_month(resolved.month())?
                .with_day(resolved.day())?
                .with_hour(resolved.hour())?
                .with_minute(resolved.minute())?
                .with_second(resolved.second())?
                .with_timezone(&Utc)
        };

        now.ok_or_else(|| anyhow!("Given datetime resolved to an unrepresentable value"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Date, RelativeDate, RelativeTime, Time};
    use chrono::{NaiveDate, NaiveTime};

    mod resolve {
        use super::*;

        fn now() -> NaiveDateTime {
            NaiveDateTime::new(
                NaiveDate::from_ymd(2012, 01, 01),
                NaiveTime::from_hms(01, 23, 45),
            )
        }

        mod given_datetime_with_date {
            use super::*;

            #[test]
            fn returns_now_with_adjusted_date() {
                let actual = DateTime {
                    date: Some(Date::Relative(RelativeDate::Days(2))),
                    time: None,
                }
                .resolve(now());

                let expected = NaiveDateTime::new(NaiveDate::from_ymd(2012, 01, 03), now().time());

                assert_eq!(expected, actual);
            }
        }

        mod given_datetime_with_time {
            use super::*;

            #[test]
            fn returns_now_with_adjusted_time() {
                let actual = DateTime {
                    date: None,
                    time: Some(Time::Relative(RelativeTime {
                        hours: Some(10),
                        minutes: Some(20),
                        ..Default::default()
                    })),
                }
                .resolve(now());

                let expected = NaiveDateTime::new(now().date(), NaiveTime::from_hms(11, 43, 45));

                assert_eq!(expected, actual);
            }
        }

        mod given_datetime_with_date_and_time {
            use super::*;

            #[test]
            fn returns_now_with_adjusted_date_and_time() {
                let actual = DateTime {
                    date: Some(Date::Relative(RelativeDate::Days(2))),
                    time: Some(Time::Relative(RelativeTime {
                        hours: Some(10),
                        minutes: Some(20),
                        ..Default::default()
                    })),
                }
                .resolve(now());

                let expected = NaiveDateTime::new(
                    NaiveDate::from_ymd(2012, 01, 03),
                    NaiveTime::from_hms(11, 43, 45),
                );

                assert_eq!(expected, actual);
            }
        }

        mod given_empty_datetime {
            use super::*;

            #[test]
            fn returns_now() {
                let actual = DateTime {
                    date: None,
                    time: None,
                }
                .resolve(now());

                let expected = now();

                assert_eq!(expected, actual);
            }
        }
    }

    mod resolve_now {
        use super::*;
        use chrono::Duration;

        mod given_relative_datetime {
            use super::*;

            #[test]
            fn resolves_it() {
                let now = Local::now();

                let actual = DateTime {
                    date: None,
                    time: Some(Time::Relative(RelativeTime {
                        hours: Some(3),
                        ..Default::default()
                    })),
                }
                .resolve_utc(now)
                .unwrap();

                let expected = now + Duration::hours(3);

                assert_eq!(expected, actual);
            }
        }

        mod given_absolute_datetime {
            use super::*;
            use chrono::Timelike;

            #[test]
            fn resolves_it() {
                let now = Local::now();

                let actual = DateTime {
                    date: None,
                    time: Some(Time::Absolute(NaiveTime::from_hms(12, 34, 56))),
                }
                .resolve_utc(now)
                .unwrap();

                let expected = now
                    .with_hour(12)
                    .unwrap()
                    .with_minute(34)
                    .unwrap()
                    .with_second(56)
                    .unwrap();

                assert_eq!(expected, actual);
            }
        }
    }
}
