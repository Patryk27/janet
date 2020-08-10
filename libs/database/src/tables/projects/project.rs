use crate::Id;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct Project {
    pub id: Id<Self>,
    pub ext_id: i64,
    pub created_at: DateTime<Utc>,
}
