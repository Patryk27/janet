use crate::gitlab::UserId;
use crate::interface::{Command, MergeRequestPtr};
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::combinator::all_consuming;
use nom::IResult;

use self::{
    date_time_spec::date_time_spec,
    id::id,
    merge_request_iid::merge_request_iid,
    merge_request_ptr::merge_request_ptr,
    name::name,
    project_id::project_id,
    project_name::project_name,
    project_ptr::project_ptr,
    url::url,
};

mod date_time_spec;
mod id;
mod merge_request_iid;
mod merge_request_ptr;
mod name;
mod project_id;
mod project_name;
mod project_ptr;
mod url;

pub fn parse(user: UserId, merge_request: MergeRequestPtr, cmd: &str) -> IResult<&str, Command> {
    all_consuming(alt((
        |i| add_merge_request_dependency(i, &user, &merge_request),
        |i| remove_merge_request_dependency(i, &user, &merge_request),
        |i| add_reminder(i, &user, &merge_request),
    )))(cmd)
}

fn add_merge_request_dependency<'a>(
    i: &'a str,
    user: &UserId,
    source: &MergeRequestPtr,
) -> IResult<&'a str, Command> {
    let (i, _) = tag_no_case("+depends on ")(i)?;
    let (i, dependency) = merge_request_ptr(i)?;

    Ok((
        i,
        Command::AddMergeRequestDependency {
            user: user.to_owned(),
            source: source.to_owned(),
            dependency,
        },
    ))
}

fn remove_merge_request_dependency<'a>(
    i: &'a str,
    user: &UserId,
    source: &MergeRequestPtr,
) -> IResult<&'a str, Command> {
    let (i, _) = tag_no_case("-depends on ")(i)?;
    let (i, dependency) = merge_request_ptr(i)?;

    Ok((
        i,
        Command::RemoveMergeRequestDependency {
            user: user.to_owned(),
            source: source.to_owned(),
            dependency,
        },
    ))
}

fn add_reminder<'a>(
    i: &'a str,
    user: &UserId,
    merge_request: &MergeRequestPtr,
) -> IResult<&'a str, Command> {
    let (i, _) = tag_no_case("+remind me ")(i)?;
    let (i, remind_at) = date_time_spec(i)?;

    Ok((
        i,
        Command::AddReminder {
            user: user.to_owned(),
            merge_request: merge_request.to_owned(),
            remind_at,
        },
    ))
}
