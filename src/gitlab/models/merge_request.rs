use crate::gitlab::MergeRequestId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MergeRequest {
    pub id: MergeRequestId,
}
