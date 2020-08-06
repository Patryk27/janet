use crate::gitlab::ProjectId;
use crate::interface::{Id, ParseAtom};
use nom::combinator::map;
use nom::IResult;

impl ParseAtom for ProjectId {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(Id::parse, |id| Self::new(id.into_inner()))(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert(expected: ProjectId, input: &str) {
        let expected = Ok(("", expected));
        let actual = ProjectId::parse(input);

        assert_eq!(expected, actual, "Input: {}", input);
    }

    #[test]
    fn test() {
        assert(ProjectId::new(1), "1");
        assert(ProjectId::new(12), "12");
        assert(ProjectId::new(123), "123");
        assert(ProjectId::new(2048), "2048");
    }
}
