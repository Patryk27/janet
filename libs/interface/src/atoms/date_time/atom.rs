use crate::{Atom, Date, DateTime, Time};
use nom::branch::alt;
use nom::character::complete::char;
use nom::{IResult, Parser};

impl Atom for DateTime {
    fn parse(i: &str) -> IResult<&str, Self> {
        let date_and_time = Date::parse
            .and(char(' '))
            .and(Time::parse)
            .map(|((date, _), time)| DateTime {
                date: Some(date),
                time: Some(time),
            });

        let date = Date::parse.map(|date| DateTime {
            date: Some(date),
            time: None,
        });

        let time = Time::parse.map(|time| DateTime {
            date: None,
            time: Some(time),
        });

        alt((date_and_time, date, time))(i)
    }
}

/// Some of the tests below don't really make that much sense from linguistic
/// point of view (usually you wouldn't say e.g. "remind me next week in 3h
/// 5m"), but our syntax is quite versatile, so we're allowing & testing them.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DayOfWeek, RelativeDate, RelativeTime};
    use chrono::{NaiveDate, NaiveTime};
    use pretty_assertions as pa;

    fn assert(input: &str, expected: DateTime) {
        let actual = DateTime::parse_unwrap(input);

        pa::assert_eq!(expected, actual, "For input: {}", input);
    }

    #[test]
    fn absolute_date_without_time() {
        assert(
            "2018-01-01",
            DateTime {
                date: Some(Date::Absolute(NaiveDate::from_ymd(2018, 01, 01))),
                time: None,
            },
        );
    }

    #[test]
    fn absolute_date_with_absolute_time() {
        for &input in &["2018-01-01 12:34", "2018-01-01 at 12:34"] {
            assert(
                input,
                DateTime {
                    date: Some(Date::Absolute(NaiveDate::from_ymd(2018, 01, 01))),
                    time: Some(Time::Absolute(NaiveTime::from_hms(12, 34, 00))),
                },
            );
        }
    }

    #[test]
    fn absolute_date_with_relative_time() {
        for &input in &["2018-01-01 3h 2m 1s", "2018-01-01 in 3h 2m 1s"] {
            assert(
                input,
                DateTime {
                    date: Some(Date::Absolute(NaiveDate::from_ymd(2018, 01, 01))),
                    time: Some(Time::Relative(RelativeTime {
                        hours: Some(3),
                        minutes: Some(2),
                        seconds: Some(1),
                    })),
                },
            );
        }
    }

    #[test]
    fn relative_date_without_time() {
        assert(
            "today",
            DateTime {
                date: Some(Date::Relative(RelativeDate::Days(0))),
                time: None,
            },
        );

        assert(
            "in 3d",
            DateTime {
                date: Some(Date::Relative(RelativeDate::Days(3))),
                time: None,
            },
        );

        for &input in &["monday", "on monday"] {
            assert(
                input,
                DateTime {
                    date: Some(Date::Relative(RelativeDate::DayOfWeek(DayOfWeek::Monday))),
                    time: None,
                },
            );
        }

        assert(
            "next week",
            DateTime {
                date: Some(Date::Relative(RelativeDate::Weeks(1))),
                time: None,
            },
        );

        assert(
            "in 3w",
            DateTime {
                date: Some(Date::Relative(RelativeDate::Weeks(3))),
                time: None,
            },
        );
    }

    #[test]
    fn relative_date_with_absolute_time() {
        for &input in &["today 12:34", "today at 12:34"] {
            assert(
                input,
                DateTime {
                    date: Some(Date::Relative(RelativeDate::Days(0))),
                    time: Some(Time::Absolute(NaiveTime::from_hms(12, 34, 00))),
                },
            );
        }

        for &input in &["3d 12:34", "in 3d 12:34", "3d at 12:34", "in 3d at 12:34"] {
            assert(
                input,
                DateTime {
                    date: Some(Date::Relative(RelativeDate::Days(3))),
                    time: Some(Time::Absolute(NaiveTime::from_hms(12, 34, 00))),
                },
            );
        }

        for &input in &[
            "monday 12:34",
            "on monday 12:34",
            "monday at 12:34",
            "on monday at 12:34",
        ] {
            assert(
                input,
                DateTime {
                    date: Some(Date::Relative(RelativeDate::DayOfWeek(DayOfWeek::Monday))),
                    time: Some(Time::Absolute(NaiveTime::from_hms(12, 34, 00))),
                },
            );
        }

        for &input in &["next week 12:34", "next week at 12:34"] {
            assert(
                input,
                DateTime {
                    date: Some(Date::Relative(RelativeDate::Weeks(1))),
                    time: Some(Time::Absolute(NaiveTime::from_hms(12, 34, 00))),
                },
            );
        }

        for &input in &["3w 12:34", "in 3w 12:34", "3w at 12:34", "in 3w at 12:34"] {
            assert(
                input,
                DateTime {
                    date: Some(Date::Relative(RelativeDate::Weeks(3))),
                    time: Some(Time::Absolute(NaiveTime::from_hms(12, 34, 00))),
                },
            );
        }
    }

    #[test]
    fn relative_date_with_relative_time() {
        for &input in &["today 3h 5m", "today in 3h 5m"] {
            assert(
                input,
                DateTime {
                    date: Some(Date::Relative(RelativeDate::Days(0))),
                    time: Some(Time::Relative(RelativeTime {
                        hours: Some(3),
                        minutes: Some(5),
                        ..Default::default()
                    })),
                },
            );
        }

        for &input in &["1d 2h 3m", "in 1d 2h 3m", "in 1d in 2h 3m"] {
            assert(
                input,
                DateTime {
                    date: Some(Date::Relative(RelativeDate::Days(1))),
                    time: Some(Time::Relative(RelativeTime {
                        hours: Some(2),
                        minutes: Some(3),
                        ..Default::default()
                    })),
                },
            );
        }

        for &input in &[
            "monday 3h 5m",
            "on monday 3h 5m",
            "monday in 3h 5m",
            "on monday in 3h 5m",
        ] {
            assert(
                input,
                DateTime {
                    date: Some(Date::Relative(RelativeDate::DayOfWeek(DayOfWeek::Monday))),
                    time: Some(Time::Relative(RelativeTime {
                        hours: Some(3),
                        minutes: Some(5),
                        ..Default::default()
                    })),
                },
            );
        }

        for &input in &["next week 3h 5m", "next week in 3h 5m"] {
            assert(
                input,
                DateTime {
                    date: Some(Date::Relative(RelativeDate::Weeks(1))),
                    time: Some(Time::Relative(RelativeTime {
                        hours: Some(3),
                        minutes: Some(5),
                        ..Default::default()
                    })),
                },
            );
        }

        for &input in &["3w 4h 5m", "in 3w 4h 5m", "3w in 4h 5m", "in 3w in 4h 5m"] {
            assert(
                input,
                DateTime {
                    date: Some(Date::Relative(RelativeDate::Weeks(3))),
                    time: Some(Time::Relative(RelativeTime {
                        hours: Some(4),
                        minutes: Some(5),
                        ..Default::default()
                    })),
                },
            );
        }
    }
}
