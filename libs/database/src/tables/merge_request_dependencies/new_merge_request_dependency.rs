use crate::{Id, MergeRequest, User};

#[derive(Clone, Debug)]
pub struct NewMergeRequestDependency {
    pub user_id: Id<User>,
    pub discussion_ext_id: String,
    pub src_merge_request_id: Id<MergeRequest>,
    pub dst_merge_request_id: Id<MergeRequest>,
}
