use crate::{Id, Project};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct MergeRequest {
    pub id: Id<Self>,
    pub project_id: Id<Project>,
    pub ext_id: i64,
    pub iid: i64,
    pub state: String,
    pub checked_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
