use super::{merge_request_iid, project_ptr, url};
use crate::interface::MergeRequestPtr;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::sequence::tuple;
use nom::IResult;

pub fn merge_request_ptr(i: &str) -> IResult<&str, MergeRequestPtr> {
    alt((merge_request_ptr_id, merge_request_ptr_url))(i)
}

fn merge_request_ptr_id(i: &str) -> IResult<&str, MergeRequestPtr> {
    let (i, (project, _, merge_request)) =
        tuple((opt(project_ptr), tag("!"), merge_request_iid))(i)?;

    Ok((
        i,
        MergeRequestPtr::Iid {
            project,
            merge_request,
        },
    ))
}

fn merge_request_ptr_url(i: &str) -> IResult<&str, MergeRequestPtr> {
    map(url, MergeRequestPtr::Url)(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gitlab::{MergeRequestIid, ProjectId, ProjectName};
    use crate::interface::ProjectPtr;

    fn assert(expected: MergeRequestPtr, input: &str) {
        assert_eq!(
            Ok(("", expected)),
            merge_request_ptr(input),
            "Input: {}",
            input
        );
    }

    #[test]
    fn test() {
        assert(
            MergeRequestPtr::Iid {
                project: None,
                merge_request: MergeRequestIid::new(456),
            },
            "!456",
        );

        assert(
            MergeRequestPtr::Iid {
                project: Some(ProjectPtr::Id(ProjectId::new(123))),
                merge_request: MergeRequestIid::new(456),
            },
            "123!456",
        );

        assert(
            MergeRequestPtr::Iid {
                project: Some(ProjectPtr::Name {
                    namespace: None,
                    name: ProjectName::new("hello-world"),
                }),
                merge_request: MergeRequestIid::new(456),
            },
            "hello-world!456",
        );

        // TODO test with namespace

        // assert_eq!( TODO
        //     (
        //         "",
        //         MergeRequestPtr::Url("https://gitlab.com/foo/bar".into())
        //     ),
        //     merge_request_ptr("https://gitlab.com/foo/bar").unwrap()
        // );
    }
}
