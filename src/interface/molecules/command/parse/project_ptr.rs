use super::{project_id, project_name};
use crate::interface::ProjectPtr;
use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;

pub fn project_ptr(i: &str) -> IResult<&str, ProjectPtr> {
    alt((project_ptr_id, project_ptr_name))(i)
}

fn project_ptr_id(i: &str) -> IResult<&str, ProjectPtr> {
    map(project_id, ProjectPtr::Id)(i)
}

fn project_ptr_name(i: &str) -> IResult<&str, ProjectPtr> {
    map(project_name, |name| ProjectPtr::Name {
        namespace: None,
        name,
    })(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gitlab::{ProjectId, ProjectName};

    fn assert(expected: ProjectPtr, input: &str) {
        assert_eq!(Ok(("", expected)), project_ptr(input), "Input: {}", input);
    }

    #[test]
    fn test() {
        assert(ProjectPtr::Id(ProjectId::new(123)), "123");

        assert(
            ProjectPtr::Name {
                namespace: None,
                name: ProjectName::new("hello-world"),
            },
            "hello-world",
        );

        // assert( TODO
        //     ProjectPtr::Name {
        //         namespace:
        // Some(NamespacePtr::Name(NamespaceName::new("somewhere-else"))),
        //         name: ProjectName::new("hello-world"),
        //     },
        //     "somewhere-else/hello-world",
        // );
    }
}
