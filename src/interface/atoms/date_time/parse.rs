use crate::interface::{Atom, DateTime};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::IResult;

impl Atom for DateTime {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((today, tomorrow))(i)
    }
}

fn today(i: &str) -> IResult<&str, DateTime> {
    let (i, _) = tag("today")(i)?;
    let (i, _) = opt(tag(" at "))(i)?;

    todo!()
}

fn tomorrow(i: &str) -> IResult<&str, DateTime> {
    let (i, _) = tag("tomorrow")(i)?;
    let (i, _) = opt(tag(" at "))(i)?;

    todo!()
}
