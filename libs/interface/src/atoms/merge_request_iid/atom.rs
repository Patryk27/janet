use crate::Atom;
use lib_gitlab::MergeRequestIid;
use nom::combinator::map;
use nom::IResult;

impl Atom for MergeRequestIid {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(usize::parse, Self::new)(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("123" => MergeRequestIid::new(123) ; "123")]
    fn test(input: &str) -> MergeRequestIid {
        MergeRequestIid::parse_unwrap(input)
    }
}
