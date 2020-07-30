use super::id;
use crate::gitlab::MergeRequestIid;
use nom::combinator::map;
use nom::IResult;

pub fn merge_request_iid(i: &str) -> IResult<&str, MergeRequestIid> {
    map(id, MergeRequestIid::new)(i)
}
