use crate::gitlab::MergeRequestIid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MergeRequest {
    pub iid: MergeRequestIid,
    pub web_url: String,
}
