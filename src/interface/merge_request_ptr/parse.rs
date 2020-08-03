use crate::gitlab::MergeRequestIid;
use crate::interface::{MergeRequestPtr, Parse, ProjectPtr};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::sequence::tuple;
use nom::IResult;

impl Parse for MergeRequestPtr {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((id, url))(i)
    }
}

fn id(i: &str) -> IResult<&str, MergeRequestPtr> {
    let (i, (project, _, merge_request)) =
        tuple((opt(ProjectPtr::parse), tag("!"), MergeRequestIid::parse))(i)?;

    Ok((
        i,
        MergeRequestPtr::Iid {
            project,
            merge_request,
        },
    ))
}

fn url(i: &str) -> IResult<&str, MergeRequestPtr> {
    map(Parse::parse, MergeRequestPtr::Url)(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gitlab::{MergeRequestIid, NamespaceName, ProjectId, ProjectName};
    use crate::interface::{NamespacePtr, ProjectPtr};
    use std::str::FromStr;
    use url::Url;

    fn assert(expected: MergeRequestPtr, input: &str) {
        let expected = Ok(("", expected));
        let actual = MergeRequestPtr::parse(input);

        assert_eq!(expected, actual, "Input: {}", input);
    }

    mod id {
        use super::*;

        mod with_project {
            use super::*;

            #[test]
            fn of_none() {
                assert(
                    MergeRequestPtr::Iid {
                        project: None,
                        merge_request: MergeRequestIid::new(456),
                    },
                    "!456",
                );
            }

            #[test]
            fn of_id() {
                assert(
                    MergeRequestPtr::Iid {
                        project: Some(ProjectPtr::Id(ProjectId::new(123))),
                        merge_request: MergeRequestIid::new(456),
                    },
                    "123!456",
                );
            }

            mod of_name {
                use super::*;

                mod with_namespace {
                    use super::*;

                    #[test]
                    fn of_none() {
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
                    }

                    #[test]
                    fn of_name() {
                        assert(
                            MergeRequestPtr::Iid {
                                project: Some(ProjectPtr::Name {
                                    namespace: Some(NamespacePtr::Name(NamespaceName::new(
                                        "somewhere",
                                    ))),
                                    name: ProjectName::new("hello-world"),
                                }),
                                merge_request: MergeRequestIid::new(456),
                            },
                            "somewhere/hello-world!456",
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn url() {
        assert(
            MergeRequestPtr::Url(
                Url::from_str("https://gitlab.com/some/project/-/merge_requests/123").unwrap(),
            ),
            "https://gitlab.com/some/project/-/merge_requests/123",
        );
    }
}
