use crate::{Atom, DayOfWeek};
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::combinator::value;
use nom::IResult;

impl Atom for DayOfWeek {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((
            value(Self::Monday, tag_no_case("monday")),
            value(Self::Tuesday, tag_no_case("tuesday")),
            value(Self::Wednesday, tag_no_case("wednesday")),
            value(Self::Thursday, tag_no_case("thursday")),
            value(Self::Friday, tag_no_case("friday")),
            value(Self::Saturday, tag_no_case("saturday")),
            value(Self::Sunday, tag_no_case("sunday")),
        ))(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("monday" => DayOfWeek::Monday ; "monday")]
    #[test_case("tuesday" => DayOfWeek::Tuesday ; "tuesday")]
    #[test_case("wednesday" => DayOfWeek::Wednesday ; "wednesday")]
    #[test_case("thursday" => DayOfWeek::Thursday ; "thursday")]
    #[test_case("friday" => DayOfWeek::Friday ; "friday")]
    #[test_case("saturday" => DayOfWeek::Saturday ; "saturday")]
    #[test_case("sunday" => DayOfWeek::Sunday ; "sunday")]
    fn lower_case(input: &str) -> DayOfWeek {
        Atom::parse_unwrap(input)
    }

    #[test_case("Monday" => DayOfWeek::Monday ; "monday")]
    #[test_case("Tuesday" => DayOfWeek::Tuesday ; "tuesday")]
    #[test_case("Wednesday" => DayOfWeek::Wednesday ; "wednesday")]
    #[test_case("Thursday" => DayOfWeek::Thursday ; "thursday")]
    #[test_case("Friday" => DayOfWeek::Friday ; "friday")]
    #[test_case("Saturday" => DayOfWeek::Saturday ; "saturday")]
    #[test_case("Sunday" => DayOfWeek::Sunday ; "sunday")]
    fn mixed_case(input: &str) -> DayOfWeek {
        Atom::parse_unwrap(input)
    }
}
