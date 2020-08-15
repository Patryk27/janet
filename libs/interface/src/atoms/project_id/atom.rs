use crate::Atom;
use lib_gitlab::ProjectId;
use nom::combinator::map;
use nom::IResult;

impl Atom for ProjectId {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(usize::parse, Self::new)(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("123" => ProjectId::new(123) ; "123")]
    fn test(input: &str) -> ProjectId {
        ProjectId::parse_unwrap(input)
    }
}
