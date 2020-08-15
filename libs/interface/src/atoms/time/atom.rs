use crate::{Atom, RelativeTime, Time};
use chrono::NaiveTime;
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::char;
use nom::combinator::opt;
use nom::{IResult, Parser};

impl Atom for Time {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((relative, absolute))(i)
    }
}

fn absolute(i: &str) -> IResult<&str, Time> {
    let (i, _) = opt(tag_no_case("at "))(i)?;

    let hour_and_minute =
        usize::parse
            .and(char(':'))
            .and(usize::parse)
            .map(|((hour, _), minute)| {
                Time::Absolute(NaiveTime::from_hms(hour as u32, minute as u32, 0))
            });

    let hour = usize::parse.map(|hour| Time::Absolute(NaiveTime::from_hms(hour as u32, 0, 0)));

    alt((hour_and_minute, hour))(i)
}

fn relative(i: &str) -> IResult<&str, Time> {
    let (i, _) = opt(tag_no_case("in "))(i)?;
    let (i, time) = RelativeTime::parse(i)?;

    Ok((i, Time::Relative(time)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("12" => Time::Absolute(NaiveTime::from_hms(12, 00, 00)) ; "12")]
    #[test_case("12:34" => Time::Absolute(NaiveTime::from_hms(12, 34, 00)) ; "12:34")]
    //
    #[test_case("at 12" => Time::Absolute(NaiveTime::from_hms(12, 00, 00)) ; "at 12")]
    #[test_case("at 12:34" => Time::Absolute(NaiveTime::from_hms(12, 34, 00)) ; "at 12:34")]
    fn absolute(input: &str) -> Time {
        Time::parse_unwrap(input)
    }

    #[test_case("12h" => Time::Relative(RelativeTime { hours: Some(12), ..Default::default() }) ; "12h")]
    #[test_case("12h 34m" => Time::Relative(RelativeTime { hours: Some(12), minutes: Some(34), ..Default::default() }) ; "12h 34m")]
    #[test_case("12h 34m 56s" => Time::Relative(RelativeTime { hours: Some(12), minutes: Some(34), seconds: Some(56) }) ; "12h 34m 56s")]
    //
    #[test_case("in 12h" => Time::Relative(RelativeTime { hours: Some(12), ..Default::default() }) ; "in 12h")]
    #[test_case("in 12h 34m" => Time::Relative(RelativeTime { hours: Some(12), minutes: Some(34), ..Default::default() }) ; "in 12h 34m")]
    #[test_case("in 12h 34m 56s" => Time::Relative(RelativeTime { hours: Some(12), minutes: Some(34), seconds: Some(56) }) ; "in 12h 34m 56s")]
    fn relative(input: &str) -> Time {
        Time::parse_unwrap(input)
    }
}
