use crate::features::prelude::*;
use crate::{MergeRequest, User};

#[derive(Clone, Debug, FromRow)]
pub struct Reminder {
    /// Internal reminder id
    pub id: Id<Self>,

    /// Internal id of the user who should get reminded
    pub user_id: Id<User>,

    /// Internal id of the merge request where we should create comment
    pub merge_request_id: Id<MergeRequest>,

    /// GitLab's discussion id where we should create comment
    pub ext_discussion_id: String,

    /// Message to remind; optional
    pub message: Option<String>,

    /// When we should remind
    pub remind_at: DateTime<Utc>,

    /// When the reminder was created
    pub created_at: DateTime<Utc>,
}

impl Reminder {
    pub fn ext_discussion_id(&self) -> gl::DiscussionId {
        gl::DiscussionId::new(&self.ext_discussion_id)
    }
}
