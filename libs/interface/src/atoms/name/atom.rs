use crate::{Atom, Name};
use nom::bytes::complete::take_while1;
use nom::combinator::map;
use nom::IResult;

impl Atom for Name {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(
            take_while1(|c: char| c.is_alphanumeric() || c == '-' || c == '_'),
            Self::new,
        )(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("a" => Name::new("a") ; "letter")]
    #[test_case("a1" => Name::new("a1") ; "letter with number")]
    #[test_case("1" => Name::new("1") ; "number")]
    #[test_case("1a" => Name::new("1a") ; "number with letter")]
    #[test_case("-" => Name::new("-") ; "dash")]
    #[test_case("_" => Name::new("_") ; "underscore")]
    #[test_case("foo" => Name::new("foo") ; "word")]
    #[test_case("foo123" => Name::new("foo123") ; "word with number")]
    #[test_case("123foo" => Name::new("123foo") ; "number with word")]
    #[test_case("foo-123" => Name::new("foo-123") ; "word with number separated with dash")]
    #[test_case("foo_123" => Name::new("foo_123") ; "word with number separated with underscore")]
    fn test(input: &str) -> Name {
        Name::parse_unwrap(input)
    }
}
