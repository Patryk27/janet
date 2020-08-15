use crate::Atom;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::IResult;

impl Atom for usize {
    fn parse(i: &str) -> IResult<&str, Self> {
        map_res(digit1, |num: &str| num.parse())(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("0" => 0 ; "0")]
    #[test_case("1" => 1 ; "1")]
    #[test_case("123" => 123 ; "123")]
    #[test_case("456" => 456 ; "456")]
    fn test(input: &str) -> usize {
        usize::parse_unwrap(input)
    }
}
