use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::IResult;

pub fn id(i: &str) -> IResult<&str, usize> {
    map_res(digit1, |num: &str| num.parse::<usize>())(i)
}
