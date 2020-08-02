use crate::gitlab::MergeRequestIid;
use crate::interface::{Id, Parse};
use nom::combinator::map;
use nom::IResult;

impl Parse for MergeRequestIid {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(Id::parse, |id| MergeRequestIid::new(id.into_inner()))(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert(expected: MergeRequestIid, input: &str) {
        let expected = Ok(("", expected));
        let actual = MergeRequestIid::parse(input);

        assert_eq!(expected, actual, "Input: {}", input);
    }

    #[test]
    fn test() {
        assert(MergeRequestIid::new(1), "1");
        assert(MergeRequestIid::new(12), "12");
        assert(MergeRequestIid::new(123), "123");
        assert(MergeRequestIid::new(2048), "2048");
    }
}
