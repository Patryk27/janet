use crate::features::prelude::*;
use crate::{MergeRequest, User};

#[derive(Clone, Debug, FromRow)]
pub struct MergeRequestDependency {
    /// Internal dependency id
    pub id: Id<Self>,

    /// Internal id of the user who should be notified on change
    pub user_id: Id<User>,

    /// GitLab's discussion id
    pub ext_discussion_id: String,

    /// Internal id of the source merge request (i.e. the one where you write
    /// the `depends on` comment)
    pub src_merge_request_id: Id<MergeRequest>,

    /// Internal id of the destination merge request (i.e. the one referred
    /// inside the `depends on` comment)
    pub dst_merge_request_id: Id<MergeRequest>,

    /// When the model was created in the database
    pub created_at: DateTime<Utc>,
}

impl MergeRequestDependency {
    pub fn ext_discussion_id(&self) -> gl::DiscussionId {
        gl::DiscussionId::new(&self.ext_discussion_id)
    }
}
