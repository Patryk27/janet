use crate::features::prelude::*;
use crate::features::LogEntry;

#[derive(Clone, Debug, Default)]
pub struct GetLogEntries;

#[async_trait]
impl Query for GetLogEntries {
    type Model = LogEntry;

    #[tracing::instrument(skip(db))]
    async fn execute(self, db: &Database) -> Result<Vec<Self::Model>> {
        tracing::debug!("Searching for log entries");

        sqlx::query_as("SELECT * FROM logs")
            .fetch_all(db.lock().await.deref_mut())
            .await
            .context("Couldn't search for log entries")
    }
}
