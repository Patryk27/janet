use crate::interface::{Atom, Id};
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::IResult;

impl Atom for Id {
    fn parse(i: &str) -> IResult<&str, Self> {
        let (i, id) = map_res(digit1, |num: &str| num.parse::<usize>())(i)?;
        Ok((i, Self::new(id)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert(expected: Id, input: &str) {
        let expected = Ok(("", expected));
        let actual = Id::parse(input);

        assert_eq!(expected, actual, "Input: {}", input);
    }

    #[test]
    fn test() {
        assert(Id::new(1), "1");
        assert(Id::new(12), "12");
        assert(Id::new(123), "123");
        assert(Id::new(2048), "2048");
    }
}
