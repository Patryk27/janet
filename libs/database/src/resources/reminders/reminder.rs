use crate::Id;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct Reminder {
    pub id: Id<Self>,
    pub user_id: i64,
    pub project_id: i64,
    pub merge_request_iid: i64,
    pub remind_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
