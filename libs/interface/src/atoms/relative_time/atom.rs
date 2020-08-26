use crate::{Atom, RelativeTime};
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::char;
use nom::multi::separated_list1;
use nom::{IResult, Parser};

impl Atom for RelativeTime {
    fn parse(i: &str) -> IResult<&str, Self> {
        components
            .map(|components| {
                let mut hours = None;
                let mut minutes = None;
                let mut seconds = None;

                for component in components {
                    match component {
                        Component::H(h) => hours = Some(h),
                        Component::M(m) => minutes = Some(m),
                        Component::S(s) => seconds = Some(s),
                    }
                }

                Self {
                    hours,
                    minutes,
                    seconds,
                }
            })
            .parse(i)
    }
}

enum Component {
    H(usize),
    M(usize),
    S(usize),
}

fn components(i: &str) -> IResult<&str, Vec<Component>> {
    separated_list1(char(' '), component)(i)
}

fn component(i: &str) -> IResult<&str, Component> {
    let h = usize::parse
        .and(tag_no_case("h"))
        .map(|(value, _)| Component::H(value));

    let m = usize::parse
        .and(tag_no_case("m"))
        .map(|(value, _)| Component::M(value));

    let s = usize::parse
        .and(tag_no_case("s"))
        .map(|(value, _)| Component::S(value));

    alt((h, m, s))(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("1h" => RelativeTime { hours: Some(1), ..Default::default() } ; "1h")]
    #[test_case("12h" => RelativeTime { hours: Some(12), ..Default::default() } ; "12h")]
    //
    #[test_case("1m" => RelativeTime { minutes: Some(1), ..Default::default() } ; "1m")]
    #[test_case("12m" => RelativeTime { minutes: Some(12), ..Default::default() } ; "12m")]
    //
    #[test_case("1s" => RelativeTime { seconds: Some(1), ..Default::default() } ; "1s")]
    #[test_case("12s" => RelativeTime { seconds: Some(12), ..Default::default() } ; "12s")]
    //
    #[test_case("12h 34m" => RelativeTime { hours: Some(12), minutes: Some(34), ..Default::default() } ; "12h 34m")]
    #[test_case("12m 34s" => RelativeTime { minutes: Some(12), seconds: Some(34), ..Default::default() } ; "12m 34s")]
    #[test_case("12h 34m 56s" => RelativeTime { hours: Some(12), minutes: Some(34), seconds: Some(56) } ; "12h 34m 56s")]
    fn test(input: &str) -> RelativeTime {
        RelativeTime::parse_unwrap(input)
    }
}
