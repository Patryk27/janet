use super::{project_id, project_name};
use crate::gitlab::ProjectPtr;
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
    map(project_name, ProjectPtr::Name)(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gitlab::{ProjectId, ProjectName};

    #[test]
    fn test() {
        assert_eq!(
            ("", ProjectPtr::Id(ProjectId::new(123))),
            project_ptr("123").unwrap(),
        );

        assert_eq!(
            ("", ProjectPtr::Name(ProjectName::new("hello-world"))),
            project_ptr("hello-world").unwrap(),
        );
    }
}
