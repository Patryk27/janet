use crate::{Atom, Date, RelativeDate};
use chrono::NaiveDate;
use nom::branch::alt;
use nom::character::complete::char;
use nom::{IResult, Parser};

impl Atom for Date {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((absolute, relative))(i)
    }
}

fn absolute(i: &str) -> IResult<&str, Date> {
    usize::parse
        .and(char('-'))
        .and(usize::parse)
        .and(char('-'))
        .and(usize::parse)
        .map(|((((year, _), month), _), day)| {
            Date::Absolute(NaiveDate::from_ymd(year as i32, month as u32, day as u32))
        })
        .parse(i)
}

fn relative(i: &str) -> IResult<&str, Date> {
    RelativeDate::parse.map(Date::Relative).parse(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DayOfWeek;
    use test_case::test_case;

    #[test_case("2018-01-02" => Date::Absolute(NaiveDate::from_ymd(2018, 01, 02)) ; "2018-01-02")]
    fn absolute(input: &str) -> Date {
        Date::parse_unwrap(input)
    }

    #[test_case("today" => Date::Relative(RelativeDate::Days(0)) ; "today")]
    #[test_case("tomorrow" => Date::Relative(RelativeDate::Days(1)) ; "tomorrow")]
    #[test_case("123d" => Date::Relative(RelativeDate::Days(123)) ; "123d")]
    #[test_case("in 123d" => Date::Relative(RelativeDate::Days(123)) ; "in 123d")]
    //
    #[test_case("tuesday" => Date::Relative(RelativeDate::DayOfWeek(DayOfWeek::Tuesday)) ; "tuesday")]
    #[test_case("on tuesday" => Date::Relative(RelativeDate::DayOfWeek(DayOfWeek::Tuesday)) ; "on tuesday")]
    //
    #[test_case("next week" => Date::Relative(RelativeDate::Weeks(1)) ; "next week")]
    #[test_case("123w" => Date::Relative(RelativeDate::Weeks(123)) ; "123w")]
    #[test_case("in 123w" => Date::Relative(RelativeDate::Weeks(123)) ; "in 123w")]
    fn relative(input: &str) -> Date {
        Date::parse_unwrap(input)
    }
}
