pub use self::{
    command::*,
    date_time_spec::*,
    event::*,
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

use nom::IResult;
use thiserror::Error;

mod command;
mod date_time_spec;
mod event;
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

pub trait Parse: Sized {
    fn parse(i: &str) -> IResult<&str, Self>;
}

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unknown command")]
    UnknownCommand,
}
