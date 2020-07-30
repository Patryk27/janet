use crate::gitlab::{MergeRequestIid, NamespaceName, ProjectId, UserId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "event_type")]
#[serde(rename_all = "snake_case")]
pub enum WebhookEvent {
    Note {
        object_attributes: WebhookNote,
        project: Option<WebhookProject>,
        merge_request: Option<WebhookMergeRequest>,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WebhookProject {
    pub id: ProjectId,
    pub namespace: NamespaceName,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WebhookMergeRequest {
    pub iid: MergeRequestIid,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WebhookNote {
    pub author_id: UserId,
    pub description: String,
}
