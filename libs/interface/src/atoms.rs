pub use self::{
    date::*,
    date_time::*,
    day_of_week::*,
    merge_request_iid::*,
    merge_request_ptr::*,
    name::*,
    namespace_ptr::*,
    project_id::*,
    project_name::*,
    project_ptr::*,
    ptr_context::*,
    relative_date::*,
    relative_time::*,
    time::*,
    url::*,
    usize::*,
};

mod date;
mod date_time;
mod day_of_week;
mod merge_request_iid;
mod merge_request_ptr;
mod name;
mod namespace_ptr;
mod project_id;
mod project_name;
mod project_ptr;
mod ptr_context;
mod relative_date;
mod relative_time;
mod time;
mod url;
mod usize;

use nom::IResult;

pub trait Atom: Sized {
    fn parse(i: &str) -> IResult<&str, Self>;

    #[cfg(test)]
    fn parse_unwrap(i: &str) -> Self {
        let (o, this) = Self::parse(i).unwrap_or_else(|err| {
            panic!(
                "Parser didn't match expected atom - for `{}`, got: {}",
                i, err,
            )
        });

        if !o.is_empty() {
            panic!(
                "Parser didn't match the entire string - for `{}`, stopped at: `{}`",
                i, o,
            );
        }

        this
    }
}
