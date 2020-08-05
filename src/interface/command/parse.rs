use crate::gitlab::{DiscussionId, UserId};
use crate::interface::{
    Command,
    CommandAction,
    DateTime,
    MergeRequestPtr,
    Parse,
    ParseError,
    ParseResult,
};
use nom::branch::alt;
use nom::bytes::complete::{tag_no_case, take_while};
use nom::character::complete::char;
use nom::combinator::{all_consuming, value};
use nom::IResult;

impl Command {
    #[tracing::instrument]
    pub fn parse(
        user: UserId,
        merge_request: MergeRequestPtr,
        discussion: DiscussionId,
        cmd: &str,
    ) -> ParseResult<Self> {
        tracing::debug!("Parsing command");

        parse(user, merge_request, discussion, cmd)
            .map(|(_, cmd)| cmd)
            .map_err(|_| ParseError::UnknownCommand)
    }
}

pub fn parse(
    user: UserId,
    merge_request: MergeRequestPtr,
    discussion: DiscussionId,
    cmd: &str,
) -> IResult<&str, Command> {
    all_consuming(alt((
        |i| hi(i, &user, &merge_request, &discussion),
        |i| merge_request_dependency(i, &user, &merge_request, &discussion),
        |i| reminder(i, &user, &merge_request, &discussion),
    )))(cmd)
}

fn action(i: &str) -> IResult<&str, CommandAction> {
    let add = value(CommandAction::Add, char('+'));
    let remove = value(CommandAction::Remove, char('-'));

    alt((add, remove))(i)
}

fn hi<'a>(
    i: &'a str,
    user: &UserId,
    merge_request: &MergeRequestPtr,
    discussion: &DiscussionId,
) -> IResult<&'a str, Command> {
    let (i, _) = alt((tag_no_case("hi"), tag_no_case("hello")))(i)?;
    let (i, _) = take_while(|c| ['.', '!', ' '].iter().any(|&c2| c2 == c))(i)?;

    Ok((
        i,
        Command::Hi {
            user: user.to_owned(),
            merge_request: merge_request.to_owned(),
            discussion: discussion.to_owned(),
        },
    ))
}

fn merge_request_dependency<'a>(
    i: &'a str,
    user: &UserId,
    source: &MergeRequestPtr,
    discussion: &DiscussionId,
) -> IResult<&'a str, Command> {
    let (i, action) = action(i)?;
    let (i, _) = tag_no_case("depends on ")(i)?;
    let (i, dependency) = MergeRequestPtr::parse(i)?;

    Ok((
        i,
        Command::MergeRequestDependency {
            action,
            user: user.to_owned(),
            source: source.to_owned(),
            discussion: discussion.to_owned(),
            dependency,
        },
    ))
}

fn reminder<'a>(
    i: &'a str,
    user: &UserId,
    merge_request: &MergeRequestPtr,
    discussion: &DiscussionId,
) -> IResult<&'a str, Command> {
    let (i, action) = action(i)?;
    let (i, _) = tag_no_case("remind me ")(i)?;
    let (i, remind_at) = DateTime::parse(i)?;

    Ok((
        i,
        Command::Reminder {
            action,
            user: user.to_owned(),
            merge_request: merge_request.to_owned(),
            discussion: discussion.to_owned(),
            remind_at,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gitlab::{MergeRequestIid, ProjectName};
    use crate::interface::ProjectPtr;
    use std::str::FromStr;
    use url::Url;

    fn user() -> UserId {
        UserId::new(1)
    }

    fn merge_request() -> MergeRequestPtr {
        MergeRequestPtr::Iid {
            project: None,
            merge_request: MergeRequestIid::new(2),
        }
    }

    fn discussion() -> DiscussionId {
        DiscussionId::new("0000")
    }

    fn assert(expected: Command, input: impl AsRef<str>) {
        let input = input.as_ref();

        let actual = Command::parse(user(), merge_request(), discussion(), input)
            .expect(&format!("Input: {}", input));

        assert_eq!(expected, actual, "Input: {}", input);
    }

    mod hi {
        use super::*;

        fn assert(input: impl AsRef<str>) {
            super::assert(
                Command::Hi {
                    user: user(),
                    merge_request: merge_request(),
                    discussion: discussion(),
                },
                input,
            );
        }

        #[test]
        fn test() {
            assert("hi");
            assert("HI");
            assert("hi.");
            assert("hi!");
            assert("hi!!");
            assert("hi !!");

            assert("hello");
            assert("HELLO");
            assert("hello.");
            assert("hello!");
            assert("hello!!");
            assert("hello !!");
        }
    }

    mod mod_merge_request_dependency {
        use super::*;

        mod with_dependency {
            use super::*;
            use test_case::test_case;

            #[test_case("+", CommandAction::Add ; "add")]
            #[test_case("-", CommandAction::Remove ; "remove")]
            fn of_url(prefix: &str, action: CommandAction) {
                assert(
                    Command::MergeRequestDependency {
                        action,
                        user: user(),
                        discussion: discussion(),
                        source: merge_request(),
                        dependency: MergeRequestPtr::Url(
                            Url::from_str("https://gitlab.com/some/project/-/merge_requests/123")
                                .unwrap(),
                        ),
                    },
                    format!(
                        "{}depends on https://gitlab.com/some/project/-/merge_requests/123",
                        prefix
                    ),
                );
            }

            mod of_iid {
                use super::*;
                use test_case::test_case;

                #[test_case("+", CommandAction::Add ; "add")]
                #[test_case("-", CommandAction::Remove ; "remove")]
                fn without_project(prefix: &str, action: CommandAction) {
                    assert(
                        Command::MergeRequestDependency {
                            action,
                            user: user(),
                            discussion: discussion(),
                            source: merge_request(),
                            dependency: MergeRequestPtr::Iid {
                                project: None,
                                merge_request: MergeRequestIid::new(123),
                            },
                        },
                        format!("{}depends on !123", prefix),
                    );
                }

                #[test_case("+", CommandAction::Add ; "add")]
                #[test_case("-", CommandAction::Remove ; "remove")]
                fn with_project(prefix: &str, action: CommandAction) {
                    assert(
                        Command::MergeRequestDependency {
                            action,
                            user: user(),
                            discussion: discussion(),
                            source: merge_request(),
                            dependency: MergeRequestPtr::Iid {
                                project: Some(ProjectPtr::Name {
                                    namespace: None,
                                    name: ProjectName::new("project"),
                                }),
                                merge_request: MergeRequestIid::new(123),
                            },
                        },
                        format!("{}depends on project!123", prefix),
                    );
                }
            }
        }
    }
}
