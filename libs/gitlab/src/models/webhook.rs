use crate::{DiscussionId, MergeRequestIid, NamespaceName, ProjectId, UserId};
use serde::Deserialize;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
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

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct WebhookProject {
    pub id: ProjectId,
    pub namespace: NamespaceName,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct WebhookMergeRequest {
    pub iid: MergeRequestIid,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct WebhookMergeRequestAttrs {
    pub action: String,
    pub iid: MergeRequestIid,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct WebhookNoteAttrs {
    pub author_id: UserId,
    pub description: String,
    pub discussion_id: DiscussionId,
}
