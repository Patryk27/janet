use crate::features::prelude::*;

#[derive(Clone, Debug, FromRow)]
pub struct LogEntry {
    /// Entry's origin; currently it's either "command" or "event"
    pub event: String,

    /// Entry's content; currently it's always a JSON object
    pub payload: String,

    /// When the entry was created
    pub created_at: DateTime<Utc>,
}
