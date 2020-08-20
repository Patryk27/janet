use crate::features::prelude::*;
use crate::Project;

#[derive(Clone, Debug, FromRow)]
pub struct MergeRequest {
    /// Internal merge request id
    pub id: Id<Self>,

    /// Internal project id
    pub project_id: Id<Project>,

    /// GitLab's merge request id
    pub ext_id: i64,

    /// GitLab's merge request incremental id
    pub ext_iid: i64,

    /// GitLab's merge request state (e.g. "opened" or "merged")
    pub ext_state: String,

    /// When the merge request was polled for the last time
    pub checked_at: DateTime<Utc>,

    /// When the model was created in the database
    pub created_at: DateTime<Utc>,
}

impl MergeRequest {
    pub fn ext_id(&self) -> gl::MergeRequestId {
        gl::MergeRequestId::new(self.ext_id as _)
    }

    pub fn ext_iid(&self) -> gl::MergeRequestIid {
        gl::MergeRequestIid::new(self.ext_iid as _)
    }
}
