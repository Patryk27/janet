use crate::database::Id;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct MergeRequestDependency {
    pub id: Id<Self>,
    pub user_id: i64,
    pub source_project_id: i64,
    pub source_merge_request_id: i64,
    pub dependency_project_id: i64,
    pub dependency_merge_request_id: i64,
    pub checked_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
