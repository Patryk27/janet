pub use self::{
    date_time::*,
    id::*,
    merge_request_iid::*,
    merge_request_ptr::*,
    name::*,
    namespace_ptr::*,
    project_id::*,
    project_name::*,
    project_ptr::*,
    ptr_context::*,
    url::*,
};

mod date_time;
mod id;
mod merge_request_iid;
mod merge_request_ptr;
mod name;
mod namespace_ptr;
mod project_id;
mod project_name;
mod project_ptr;
mod ptr_context;
mod url;

use nom::IResult;

pub trait Atom: Sized {
    fn parse(i: &str) -> IResult<&str, Self>;

    #[cfg(test)]
    fn parse_unwrap(i: &str) -> Self {
        Self::parse(i).unwrap().1
    }
}
