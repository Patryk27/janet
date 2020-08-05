use crate::gitlab::MergeRequestIid;
use crate::interface::ProjectPtr;
use serde::Serialize;
use url::Url;

mod parse;
mod resolve;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum MergeRequestPtr {
    Iid {
        project: Option<ProjectPtr>,
        merge_request: MergeRequestIid,
    },

    Url(Url),
}
