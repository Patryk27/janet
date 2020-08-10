use crate::{Atom, CommandAction};
use nom::branch::alt;
use nom::character::complete::{anychar, char};
use nom::combinator::{peek, value};
use nom::IResult;

impl Atom for CommandAction {
    fn parse(i: &str) -> IResult<&str, Self> {
        let remove = value(CommandAction::Remove, char('-'));
        let add = value(CommandAction::Add, peek(anychar));

        alt((remove, add))(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("foo", ("foo", CommandAction::Add) ; "without prefix")]
    #[test_case("+foo", ("+foo", CommandAction::Add) ; "with plus prefix")]
    #[test_case("-foo", ("foo", CommandAction::Remove) ; "with minus prefix")]
    fn test(input: &str, output: (&str, CommandAction)) {
        assert_eq!(Ok(output), CommandAction::parse(input));
    }
}
