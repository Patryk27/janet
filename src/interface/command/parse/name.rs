use nom::bytes::complete::take_while1;
use nom::combinator::map;
use nom::IResult;

pub fn name(i: &str) -> IResult<&str, String> {
    map(
        take_while1(|c: char| c.is_alphanumeric() || c == '-' || c == '_'),
        Into::into,
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        for &case in &["foo", "foo123", "foo-123", "foo-123_321", "FoOo"] {
            assert_eq!(("", String::from(case)), name(case).unwrap());
        }
    }
}
