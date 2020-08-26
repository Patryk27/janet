use crate::Date;
use chrono::NaiveDate;

impl Date {
    pub fn resolve(&self, now: NaiveDate) -> NaiveDate {
        match self {
            Self::Absolute(date) => *date,
            Self::Relative(date) => date.resolve(now),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod given_absolute_date {
        use super::*;

        #[test]
        fn resolves_it() {
            let actual = Date::Absolute(NaiveDate::from_ymd(2020, 03, 11))
                .resolve(NaiveDate::from_ymd(2012, 01, 01));

            let expected = NaiveDate::from_ymd(2020, 03, 11);

            assert_eq!(expected, actual);
        }
    }

    mod given_relative_date {
        use super::*;
        use crate::RelativeDate;

        #[test]
        fn resolves_it() {
            let actual =
                Date::Relative(RelativeDate::Days(3)).resolve(NaiveDate::from_ymd(2012, 01, 01));

            let expected = NaiveDate::from_ymd(2012, 01, 04);

            assert_eq!(expected, actual);
        }
    }
}
