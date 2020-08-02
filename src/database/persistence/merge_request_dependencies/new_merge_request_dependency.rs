#[derive(Clone, Debug)]
pub struct NewMergeRequestDependency {
    pub user_id: i64,
    pub source_project_id: i64,
    pub source_merge_request_iid: i64,
    pub source_discussion_id: String,
    pub dependency_project_id: i64,
    pub dependency_merge_request_iid: i64,
}
