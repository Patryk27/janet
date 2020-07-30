use crate::gitlab::{MergeRequest, Note, Project};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "event_type")]
#[serde(rename_all = "snake_case")]
pub enum WebhookEvent {
    Note {
        object_attributes: Note,
        project: Option<Project>,
        merge_request: Option<MergeRequest>,
    },
}
