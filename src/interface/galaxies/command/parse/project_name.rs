use super::name;
use crate::gitlab::ProjectName;
use nom::combinator::map;
use nom::IResult;

pub fn project_name(i: &str) -> IResult<&str, ProjectName> {
    map(name, ProjectName::new)(i)
}
