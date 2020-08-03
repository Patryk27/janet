use crate::database::{Id, MergeRequest, User};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct MergeRequestDependency {
    pub id: Id<Self>,
    pub user_id: Id<User>,
    pub discussion_ext_id: String,
    pub src_merge_request_id: Id<MergeRequest>,
    pub dst_merge_request_id: Id<MergeRequest>,
    pub checked_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
