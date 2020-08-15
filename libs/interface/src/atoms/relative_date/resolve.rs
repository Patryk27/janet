use crate::RelativeDate;
use chrono::{Datelike, Duration, NaiveDate};
use std::cmp::Ordering;

impl RelativeDate {
    pub fn resolve(self, now: NaiveDate) -> NaiveDate {
        match self {
            Self::Days(days) => now + Duration::days(days as i64),

            Self::DayOfWeek(dow) => {
                let src_weekday = now.weekday().number_from_monday() as i64;
                let dst_weekday = dow.number_from_monday() as i64;

                match src_weekday.cmp(&dst_weekday) {
                    // e.g. src = Monday, dst = Wednesday
                    Ordering::Less => now + Duration::days(dst_weekday - src_weekday),

                    // e.g. src = Monday, dst = Monday
                    Ordering::Equal => now + Duration::weeks(1),

                    // e.g. src = Wednesday, dst = Monday
                    Ordering::Greater => now + Duration::days(7 - src_weekday + dst_weekday),
                }
            }

            Self::Weeks(weeks) => now + Duration::weeks(weeks as i64),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod given_relative_days {
        use super::*;

        #[test]
        fn returns_current_date_plus_that_number_of_days() {
            let actual = RelativeDate::Days(3).resolve(NaiveDate::from_ymd(2012, 01, 01));
            let expected = NaiveDate::from_ymd(2012, 01, 04);

            assert_eq!(expected, actual);
        }
    }

    mod given_relative_day_of_week {
        use super::*;
        use crate::DayOfWeek;

        mod when_today_is_that_day_of_week {
            use super::*;

            #[test]
            fn returns_current_date_plus_seven_days() {
                let actual = RelativeDate::DayOfWeek(DayOfWeek::Sunday)
                    .resolve(NaiveDate::from_ymd(2012, 01, 01));

                let expected = NaiveDate::from_ymd(2012, 01, 08);

                assert_eq!(expected, actual);
            }
        }

        mod when_today_is_not_that_day {
            use super::*;

            #[test]
            fn returns_the_next_closest_date_to_given_day_of_week_from_today() {
                // Case 1: Sunday (2012-01-01) -> Tuesday (2012-01-03)
                {
                    let actual = RelativeDate::DayOfWeek(DayOfWeek::Tuesday)
                        .resolve(NaiveDate::from_ymd(2012, 01, 01));

                    let expected = NaiveDate::from_ymd(2012, 01, 03);

                    assert_eq!(expected, actual);
                }

                // Case 2: Tuesday (2012-01-03) -> Sunday (2012-01-08)
                {
                    let actual = RelativeDate::DayOfWeek(DayOfWeek::Sunday)
                        .resolve(NaiveDate::from_ymd(2012, 01, 03));

                    let expected = NaiveDate::from_ymd(2012, 01, 08);

                    assert_eq!(expected, actual);
                }
            }
        }
    }

    mod given_relative_weeks {
        use super::*;

        #[test]
        fn returns_current_date_plus_that_number_of_weeks() {
            let actual = RelativeDate::Weeks(3).resolve(NaiveDate::from_ymd(2012, 01, 01));
            let expected = NaiveDate::from_ymd(2012, 01, 22);

            assert_eq!(expected, actual);
        }
    }
}
