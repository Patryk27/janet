use super::{merge_request_id, project_ptr, url};
use crate::gitlab::MergeRequestPtr;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

pub fn merge_request_ptr(i: &str) -> IResult<&str, MergeRequestPtr> {
    alt((merge_request_ptr_id, merge_request_ptr_url))(i)
}

fn merge_request_ptr_id(i: &str) -> IResult<&str, MergeRequestPtr> {
    let (i, (project, _, merge_request)) = tuple((project_ptr, tag("!"), merge_request_id))(i)?;

    Ok((
        i,
        MergeRequestPtr::Id {
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
    use crate::gitlab::{MergeRequestId, ProjectId, ProjectName, ProjectPtr};

    #[test]
    fn test() {
        assert_eq!(
            (
                "",
                MergeRequestPtr::Id {
                    project: ProjectPtr::Id(ProjectId::new(123)),
                    merge_request: MergeRequestId::new(456)
                }
            ),
            merge_request_ptr("123!456").unwrap()
        );

        assert_eq!(
            (
                "",
                MergeRequestPtr::Id {
                    project: ProjectPtr::Name(ProjectName::new("hello-world")),
                    merge_request: MergeRequestId::new(456)
                }
            ),
            merge_request_ptr("hello-world!456").unwrap()
        );

        // assert_eq!( TODO
        //     (
        //         "",
        //         MergeRequestPtr::Url("https://gitlab.com/foo/bar".into())
        //     ),
        //     merge_request_ptr("https://gitlab.com/foo/bar").unwrap()
        // );
    }
}
