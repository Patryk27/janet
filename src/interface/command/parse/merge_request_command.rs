use super::action;
use crate::gitlab::{DiscussionId, UserId};
use crate::interface::{
    CommandAction,
    MergeRequestCommand,
    MergeRequestCommandContext,
    MergeRequestPtr,
    ParseAtom,
};
use nom::branch::alt;
use nom::bytes::complete::{tag_no_case, take_while};
use nom::character::complete::char;
use nom::combinator::{all_consuming, value};
use nom::IResult;

pub fn parse(cmd: &str) -> IResult<&str, MergeRequestCommand> {
    all_consuming(alt((hi, manage_dependency, manage_reminder)))(cmd)
}

fn hi(i: &str) -> IResult<&str, MergeRequestCommand> {
    let (i, _) = alt((tag_no_case("hi"), tag_no_case("hello")))(i)?;
    let (i, _) = take_while(|c| ['.', '!', ' '].iter().any(|&c2| c2 == c))(i)?;

    Ok((i, MergeRequestCommand::Hi))
}

fn manage_dependency(i: &str) -> IResult<&str, MergeRequestCommand> {
    let (i, action) = action(i)?;
    let (i, _) = tag_no_case("depends on ")(i)?;
    let (i, dependency) = MergeRequestPtr::parse(i)?;

    Ok((
        i,
        MergeRequestCommand::ManageDependency { action, dependency },
    ))
}

fn manage_reminder(i: &str) -> IResult<&str, MergeRequestCommand> {
    let (i, action) = action(i)?;
    let (i, _) = tag_no_case("remind me ")(i)?;
    let (i, remind_at) = DateTime::parse(i)?;

    Ok((i, MergeRequestCommand::ManageReminder { action, remind_at }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gitlab::{MergeRequestIid, ProjectName};
    use crate::interface::ProjectPtr;
    use std::str::FromStr;
    use url::Url;

    fn assert(expected: MergeRequestCommand, input: impl AsRef<str>) {
        let input = input.as_ref();
        let actual = parse(input).expect(&format!("Input: {}", input));

        assert_eq!(expected, actual, "Input: {}", input);
    }

    mod hi {
        use super::*;

        fn assert(input: impl AsRef<str>) {
            super::assert(MergeRequestCommand::Hi, input);
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

    mod manage_dependency {
        use super::*;

        mod with_dependency {
            use super::*;
            use test_case::test_case;

            #[test_case("+", CommandAction::Add ; "add")]
            #[test_case("-", CommandAction::Remove ; "remove")]
            fn of_url(prefix: &str, action: CommandAction) {
                assert(
                    MergeRequestCommand::ManageDependency {
                        action,
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
                        MergeRequestCommand::ManageDependency {
                            action,
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
                        MergeRequestCommand::ManageDependency {
                            action,
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
