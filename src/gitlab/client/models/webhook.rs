use crate::gitlab::{DiscussionId, MergeRequestIid, NamespaceName, ProjectId, UserId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "event_type")]
#[serde(rename_all = "snake_case")]
pub enum WebhookEvent {
    MergeRequest {
        project: WebhookProject,
        object_attributes: WebhookMergeRequestAttrs,
    },

    Note {
        project: WebhookProject,
        merge_request: WebhookMergeRequest,
        object_attributes: WebhookNoteAttrs,
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
pub struct WebhookMergeRequestAttrs {
    pub action: String,
    pub iid: MergeRequestIid,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WebhookNoteAttrs {
    pub author_id: UserId,
    pub description: String,
    pub discussion_id: DiscussionId,
}
