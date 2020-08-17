use crate::{Atom, Name};
use lib_gitlab::ProjectName;
use nom::combinator::map;
use nom::IResult;

impl Atom for ProjectName {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(Name::parse, |name| Self::new(name.into_inner()))(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert(expected: ProjectName, input: &str) {
        let expected = Ok(("", expected));
        let actual = ProjectName::parse(input);

        assert_eq!(expected, actual, "Input: {}", input);
    }

    #[test]
    fn test() {
        assert(ProjectName::new("hello"), "hello");
        assert(ProjectName::new("hello-world"), "hello-world");
        assert(ProjectName::new("hello-world__123"), "hello-world__123");
    }
}
