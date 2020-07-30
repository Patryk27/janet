use super::id;
use crate::gitlab::MergeRequestId;
use nom::combinator::map;
use nom::IResult;

pub fn merge_request_id(i: &str) -> IResult<&str, MergeRequestId> {
    map(id, MergeRequestId::new)(i)
}
