use crate::gitlab::{MergeRequestId, MergeRequestIid, ProjectId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MergeRequest {
    pub id: MergeRequestId,
    pub iid: MergeRequestIid,
    pub project_id: ProjectId,
    pub state: String,
    pub web_url: String,
}
