use crate::{Atom, CommandAction, DateTime, MergeRequestCommand, MergeRequestPtr};
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case, take_while};
use nom::combinator::{all_consuming, opt, rest};
use nom::{IResult, Parser};

pub fn parse(cmd: &str) -> IResult<&str, MergeRequestCommand> {
    all_consuming(alt((hi, manage_dependency, manage_reminder)))(cmd)
}

fn hi(i: &str) -> IResult<&str, MergeRequestCommand> {
    let (i, _) = alt((tag_no_case("hi"), tag_no_case("hello")))(i)?;
    let (i, _) = take_while(|c| ['.', '!', ' '].iter().any(|&c2| c2 == c))(i)?;

    Ok((i, MergeRequestCommand::Hi))
}

fn manage_dependency(i: &str) -> IResult<&str, MergeRequestCommand> {
    CommandAction::parse
        .and(tag_no_case("depends on "))
        .and(MergeRequestPtr::parse)
        .map(
            |((action, _), dependency)| MergeRequestCommand::ManageDependency {
                action,
                dependency,
            },
        )
        .parse(i)
}

fn manage_reminder(i: &str) -> IResult<&str, MergeRequestCommand> {
    tag_no_case("remind ")
        .and(opt(tag_no_case("me ")))
        .and(DateTime::parse)
        .and(opt(tag(":").and(rest)))
        .map(|(((_, _), remind_at), message)| {
            let message = message.map(|(_, message)| message.trim().to_string());

            MergeRequestCommand::ManageReminder { remind_at, message }
        })
        .parse(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Date, ProjectPtr, RelativeDate, Time};
    use chrono::NaiveTime;
    use lib_gitlab::{MergeRequestIid, ProjectName};
    use std::str::FromStr;
    use url::Url;

    fn assert(expected: MergeRequestCommand, input: impl AsRef<str>) {
        let input = input.as_ref();
        let actual = parse(input).expect(&format!("Input: {}", input)).1;

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

            #[test_case("", CommandAction::Add ; "add")]
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

                #[test_case("", CommandAction::Add ; "add")]
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

                #[test_case("", CommandAction::Add ; "add")]
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

    mod manage_reminder {
        use super::*;
        use test_case::test_case;

        #[test_case("remind tomorrow at 12: important important!" ; "without me")]
        #[test_case("remind me tomorrow at 12: important important!" ; "with me")]
        fn with_message(input: &str) {
            assert(
                MergeRequestCommand::ManageReminder {
                    remind_at: DateTime {
                        date: Some(Date::Relative(RelativeDate::Days(1))),
                        time: Some(Time::Absolute(NaiveTime::from_hms(12, 00, 00))),
                    },
                    message: Some("important important!".to_string()),
                },
                input,
            );
        }

        #[test_case("remind tomorrow at 12" ; "without me")]
        #[test_case("remind me tomorrow at 12" ; "with me")]
        fn without_message(input: &str) {
            assert(
                MergeRequestCommand::ManageReminder {
                    remind_at: DateTime {
                        date: Some(Date::Relative(RelativeDate::Days(1))),
                        time: Some(Time::Absolute(NaiveTime::from_hms(12, 00, 00))),
                    },
                    message: None,
                },
                input,
            );
        }
    }
}
