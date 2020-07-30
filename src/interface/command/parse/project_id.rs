use super::id;
use crate::gitlab::ProjectId;
use nom::combinator::map;
use nom::IResult;

pub fn project_id(i: &str) -> IResult<&str, ProjectId> {
    map(id, ProjectId::new)(i)
}
