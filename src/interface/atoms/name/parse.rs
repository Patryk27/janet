use crate::interface::{Name, ParseAtom};
use nom::bytes::complete::take_while1;
use nom::combinator::map;
use nom::IResult;

impl ParseAtom for Name {
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

    fn assert(input: &str) {
        let expected = Ok(("", Name::new(input)));
        let actual = Name::parse(input);

        assert_eq!(expected, actual, "Input: {}", input);
    }

    #[test]
    fn test() {
        assert("a");
        assert("1");

        assert("foo");
        assert("foo123");
        assert("123foo");

        assert("foo-123");
        assert("foo_123");

        assert("123-foo");
        assert("123_foo");

        assert("FoFo");
    }
}
