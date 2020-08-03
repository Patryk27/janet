use crate::gitlab::{MergeRequestId, MergeRequestIid};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MergeRequest {
    pub id: MergeRequestId,
    pub iid: MergeRequestIid,
    pub state: String,
    pub web_url: String,
}
