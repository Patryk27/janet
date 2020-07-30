use crate::gitlab::{MergeRequestPtr, UserId};
use crate::interface::Command;
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::sequence::tuple;
use nom::IResult;

use self::{
    id::id,
    merge_request_id::merge_request_id,
    merge_request_ptr::merge_request_ptr,
    name::name,
    project_id::project_id,
    project_name::project_name,
    project_ptr::project_ptr,
    url::url,
};

mod id;
mod merge_request_id;
mod merge_request_ptr;
mod name;
mod project_id;
mod project_name;
mod project_ptr;
mod url;

pub fn parse(user: UserId, merge_request: MergeRequestPtr, cmd: &str) -> IResult<&str, Command> {
    alt((
        add_merge_request_dependency(&user, &merge_request),
        remove_merge_request_dependency(&user, &merge_request),
    ))(cmd)
}

fn add_merge_request_dependency<'a>(
    user: &'a UserId,
    source: &'a MergeRequestPtr,
) -> impl Fn(&str) -> IResult<&str, Command> + 'a {
    move |i| {
        let (i, (_, dependency)) = tuple((tag_no_case("+depends on "), merge_request_ptr))(i)?;

        Ok((
            i,
            Command::AddMergeRequestDependency {
                user: user.to_owned(),
                source: source.to_owned(),
                dependency,
            },
        ))
    }
}

fn remove_merge_request_dependency<'a>(
    user: &'a UserId,
    source: &'a MergeRequestPtr,
) -> impl Fn(&str) -> IResult<&str, Command> + 'a {
    move |i| {
        let (i, (_, dependency)) = tuple((tag_no_case("-depends on "), merge_request_ptr))(i)?;

        Ok((
            i,
            Command::RemoveMergeRequestDependency {
                user: user.to_owned(),
                source: source.to_owned(),
                dependency,
            },
        ))
    }
}
