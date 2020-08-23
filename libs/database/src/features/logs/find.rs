use crate::features::prelude::*;
use crate::features::LogEntry;

#[derive(Clone, Debug, Default)]
pub struct FindLogEntries;

#[async_trait]
impl Query for FindLogEntries {
    type Model = LogEntry;

    #[tracing::instrument(skip(db))]
    async fn execute(self, db: &Database) -> Result<Vec<Self::Model>> {
        tracing::debug!("Finding log entries");

        sqlx::query_as("SELECT * FROM logs")
            .fetch_all(db.lock().await.deref_mut())
            .await
            .context("Couldn't find log entries")
    }
}
