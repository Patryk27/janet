pub use self::{action::*, merge_request::*};

mod action;
mod merge_request;

use serde::Serialize;

/// A generic command accepted by Janet.
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum Command {
    MergeRequest {
        ctxt: MergeRequestCommandContext,
        cmd: MergeRequestCommand,
    },
}
