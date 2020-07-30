use chrono::{DateTime, Utc};

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct Log {
    pub event: String,
    pub payload: String,
    pub created_at: DateTime<Utc>,
}
