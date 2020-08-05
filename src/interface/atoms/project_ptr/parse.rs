use crate::gitlab::{NamespaceName, ProjectId, ProjectName};
use crate::interface::{Name, NamespacePtr, Parse, ProjectPtr};
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::IResult;

impl Parse for ProjectPtr {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((id, name))(i)
    }
}

fn id(i: &str) -> IResult<&str, ProjectPtr> {
    map(ProjectId::parse, ProjectPtr::Id)(i)
}

fn name(i: &str) -> IResult<&str, ProjectPtr> {
    let (i, mut path) = separated_list1(char('/'), Name::parse)(i)?;

    let (namespace, name) = if path.len() == 1 {
        let namespace = None;
        let name = ProjectName::new(path.swap_remove(0).into_inner());

        (namespace, name)
    } else {
        let name = ProjectName::new(path.remove(path.len() - 1).into_inner());

        let path = path
            .into_iter()
            .map(Name::into_inner)
            .collect::<Vec<_>>()
            .join("/");

        let namespace = Some(NamespacePtr::Name(NamespaceName::new(path)));

        (namespace, name)
    };

    Ok((i, ProjectPtr::Name { namespace, name }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gitlab::{NamespaceName, ProjectId, ProjectName};

    fn assert(expected: ProjectPtr, input: &str) {
        let expected = Ok(("", expected));
        let actual = ProjectPtr::parse(input);

        assert_eq!(expected, actual, "Input: {}", input);
    }

    #[test]
    fn id() {
        assert(ProjectPtr::Id(ProjectId::new(123)), "123");
    }

    mod name {
        use super::*;

        mod with_namespace {
            use super::*;

            #[test]
            fn of_none() {
                assert(
                    ProjectPtr::Name {
                        namespace: None,
                        name: ProjectName::new("hello-world"),
                    },
                    "hello-world",
                );
            }

            #[test]
            fn of_name() {
                assert(
                    ProjectPtr::Name {
                        namespace: Some(NamespacePtr::Name(NamespaceName::new("somewhere-else"))),
                        name: ProjectName::new("hello-world"),
                    },
                    "somewhere-else/hello-world",
                );

                assert(
                    ProjectPtr::Name {
                        namespace: Some(NamespacePtr::Name(NamespaceName::new("somewhere/else"))),
                        name: ProjectName::new("hello-world"),
                    },
                    "somewhere/else/hello-world",
                );

                assert(
                    ProjectPtr::Name {
                        namespace: Some(NamespacePtr::Name(NamespaceName::new(
                            "somewhere/completely/else",
                        ))),
                        name: ProjectName::new("hello-world"),
                    },
                    "somewhere/completely/else/hello-world",
                );
            }
        }
    }
}
