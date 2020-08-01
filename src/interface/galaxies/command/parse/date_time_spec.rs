use crate::interface::DateTimeSpec;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::IResult;

pub fn date_time_spec(i: &str) -> IResult<&str, DateTimeSpec> {
    alt((date_time_spec_today, date_time_spec_tomorrow))(i)
}

fn date_time_spec_today(i: &str) -> IResult<&str, DateTimeSpec> {
    let (i, _) = tag("today")(i)?;
    let (i, _) = opt(tag(" at "))(i)?;

    todo!()
}

fn date_time_spec_tomorrow(i: &str) -> IResult<&str, DateTimeSpec> {
    let (i, _) = tag("today")(i)?;
    let (i, _) = opt(tag(" at "))(i)?;

    todo!()
}
