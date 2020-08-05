use nom::IResult;
use thiserror::Error;

pub trait Parse: Sized {
    fn parse(i: &str) -> IResult<&str, Self>;

    #[cfg(test)]
    fn do_parse(i: &str) -> Self {
        Self::parse(i).unwrap().1
    }
}

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unknown command")]
    UnknownCommand,
}
