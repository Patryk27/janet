use crate::gitlab::UserId;
use crate::interface::{Command, DateTimeSpec, MergeRequestPtr, Parse, ParseError, ParseResult};
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::combinator::all_consuming;
use nom::IResult;

impl Command {
    pub fn parse(user: UserId, merge_request: MergeRequestPtr, cmd: &str) -> ParseResult<Self> {
        log::debug!(
            "parse(); cmd={}, user={}, merge_request={:?}",
            cmd,
            user.inner(),
            merge_request,
        );

        parse(user, merge_request, cmd)
            .map(|(_, cmd)| cmd)
            .map_err(|_| ParseError::UnknownCommand)
    }
}

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
    let (i, dependency) = MergeRequestPtr::parse(i)?;

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
    let (i, dependency) = MergeRequestPtr::parse(i)?;

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
    let (i, remind_at) = DateTimeSpec::parse(i)?;

    Ok((
        i,
        Command::AddReminder {
            user: user.to_owned(),
            merge_request: merge_request.to_owned(),
            remind_at,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gitlab::{MergeRequestIid, ProjectName};
    use crate::interface::{ProjectPtr, Url};

    fn user() -> UserId {
        UserId::new(1)
    }

    fn merge_request() -> MergeRequestPtr {
        MergeRequestPtr::Iid {
            project: None,
            merge_request: MergeRequestIid::new(2),
        }
    }

    fn assert(expected: Command, input: &str) {
        let actual = Command::parse(user(), merge_request(), input).unwrap();

        assert_eq!(expected, actual, "Input: {}", input);
    }

    mod add_merge_request_dependency {
        use super::*;

        mod with_dependency {
            use super::*;

            mod of_iid {
                use super::*;

                #[test]
                fn without_project() {
                    assert(
                        Command::AddMergeRequestDependency {
                            user: user(),
                            source: merge_request(),
                            dependency: MergeRequestPtr::Iid {
                                project: None,
                                merge_request: MergeRequestIid::new(123),
                            },
                        },
                        "+depends on !123",
                    );
                }

                #[test]
                fn with_project() {
                    assert(
                        Command::AddMergeRequestDependency {
                            user: user(),
                            source: merge_request(),
                            dependency: MergeRequestPtr::Iid {
                                project: Some(ProjectPtr::Name {
                                    namespace: None,
                                    name: ProjectName::new("project"),
                                }),
                                merge_request: MergeRequestIid::new(123),
                            },
                        },
                        "+depends on project!123",
                    );
                }
            }

            #[test]
            fn of_url() {
                assert(
                    Command::AddMergeRequestDependency {
                        user: user(),
                        source: merge_request(),
                        dependency: MergeRequestPtr::Url(Url::new(
                            "https://gitlab.com/some/project/-/merge_requests/123",
                        )),
                    },
                    "+depends on https://gitlab.com/some/project/-/merge_requests/123",
                );
            }
        }
    }

    mod remove_merge_request_dependency {
        use super::*;

        mod with_dependency {
            use super::*;

            mod of_iid {
                use super::*;

                #[test]
                fn without_project() {
                    assert(
                        Command::RemoveMergeRequestDependency {
                            user: user(),
                            source: merge_request(),
                            dependency: MergeRequestPtr::Iid {
                                project: None,
                                merge_request: MergeRequestIid::new(123),
                            },
                        },
                        "-depends on !123",
                    );
                }

                #[test]
                fn with_project() {
                    assert(
                        Command::RemoveMergeRequestDependency {
                            user: user(),
                            source: merge_request(),
                            dependency: MergeRequestPtr::Iid {
                                project: Some(ProjectPtr::Name {
                                    namespace: None,
                                    name: ProjectName::new("project"),
                                }),
                                merge_request: MergeRequestIid::new(123),
                            },
                        },
                        "-depends on project!123",
                    );
                }
            }

            #[test]
            fn of_url() {
                assert(
                    Command::RemoveMergeRequestDependency {
                        user: user(),
                        source: merge_request(),
                        dependency: MergeRequestPtr::Url(Url::new(
                            "https://gitlab.com/some/project/-/merge_requests/123",
                        )),
                    },
                    "-depends on https://gitlab.com/some/project/-/merge_requests/123",
                );
            }
        }
    }
}
