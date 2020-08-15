use crate::{Atom, DayOfWeek, RelativeDate};
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::combinator::{opt, value};
use nom::{IResult, Parser};

impl Atom for RelativeDate {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((day, day_of_week, week))(i)
    }
}

fn day(i: &str) -> IResult<&str, RelativeDate> {
    let predefined = alt((
        value(RelativeDate::Days(0), tag_no_case("today")),
        value(RelativeDate::Days(1), tag_no_case("tomorrow")),
        value(RelativeDate::Days(2), tag_no_case("the day after tomorrow")),
    ));

    let dynamic = opt(tag_no_case("in "))
        .and(usize::parse)
        .and(tag_no_case("d"))
        .map(|((_, days), _)| RelativeDate::Days(days));

    alt((predefined, dynamic))(i)
}

fn day_of_week(i: &str) -> IResult<&str, RelativeDate> {
    opt(tag_no_case("on "))
        .and(DayOfWeek::parse)
        .map(|(_, weekday)| RelativeDate::DayOfWeek(weekday))
        .parse(i)
}

fn week(i: &str) -> IResult<&str, RelativeDate> {
    let predefined = value(RelativeDate::Weeks(1), tag_no_case("next week"));

    let dynamic = opt(tag_no_case("in "))
        .and(usize::parse)
        .and(tag_no_case("w"))
        .map(|((_, weeks), _)| RelativeDate::Weeks(weeks));

    alt((predefined, dynamic))(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("today" => RelativeDate::Days(0) ; "today")]
    #[test_case("tomorrow" => RelativeDate::Days(1) ; "tomorrow")]
    #[test_case("the day after tomorrow" => RelativeDate::Days(2) ; "the day after tomorrow")]
    //
    #[test_case("1d" => RelativeDate::Days(1) ; "1d")]
    #[test_case("in 1d" => RelativeDate::Days(1) ; "in 1d")]
    //
    #[test_case("123d" => RelativeDate::Days(123) ; "123d")]
    #[test_case("in 123d" => RelativeDate::Days(123) ; "in 123d")]
    fn day(input: &str) -> RelativeDate {
        RelativeDate::parse_unwrap(input)
    }

    #[test_case("monday" => RelativeDate::DayOfWeek(DayOfWeek::Monday) ; "monday")]
    #[test_case("tuesday" => RelativeDate::DayOfWeek(DayOfWeek::Tuesday) ; "tuesday")]
    #[test_case("wednesday" => RelativeDate::DayOfWeek(DayOfWeek::Wednesday) ; "wednesday")]
    #[test_case("thursday" => RelativeDate::DayOfWeek(DayOfWeek::Thursday) ; "thursday")]
    #[test_case("friday" => RelativeDate::DayOfWeek(DayOfWeek::Friday) ; "friday")]
    #[test_case("saturday" => RelativeDate::DayOfWeek(DayOfWeek::Saturday) ; "saturday")]
    #[test_case("sunday" => RelativeDate::DayOfWeek(DayOfWeek::Sunday) ; "sunday")]
    //
    #[test_case("on monday" => RelativeDate::DayOfWeek(DayOfWeek::Monday) ; "on monday")]
    #[test_case("on tuesday" => RelativeDate::DayOfWeek(DayOfWeek::Tuesday) ; "on tuesday")]
    #[test_case("on wednesday" => RelativeDate::DayOfWeek(DayOfWeek::Wednesday) ; "on wednesday")]
    #[test_case("on thursday" => RelativeDate::DayOfWeek(DayOfWeek::Thursday) ; "on thursday")]
    #[test_case("on friday" => RelativeDate::DayOfWeek(DayOfWeek::Friday) ; "on friday")]
    #[test_case("on saturday" => RelativeDate::DayOfWeek(DayOfWeek::Saturday) ; "on saturday")]
    #[test_case("on sunday" => RelativeDate::DayOfWeek(DayOfWeek::Sunday) ; "on sunday")]
    fn day_of_week(input: &str) -> RelativeDate {
        RelativeDate::parse_unwrap(input)
    }

    #[test_case("next week" => RelativeDate::Weeks(1) ; "next week")]
    //
    #[test_case("1w" => RelativeDate::Weeks(1) ; "1w")]
    #[test_case("in 1w" => RelativeDate::Weeks(1) ; "in 1w")]
    //
    #[test_case("123w" => RelativeDate::Weeks(123) ; "123w")]
    #[test_case("in 123w" => RelativeDate::Weeks(123) ; "in 123w")]
    fn week(input: &str) -> RelativeDate {
        RelativeDate::parse_unwrap(input)
    }
}
