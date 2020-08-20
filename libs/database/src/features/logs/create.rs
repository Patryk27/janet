use crate::features::prelude::*;

#[derive(Clone, Debug)]
pub struct CreateLogEntry {
    /// Entry's origin; currently it's either "command" or "event"
    pub event: String,

    /// Entry's content; currently it's always a JSON object
    pub payload: String,
}

#[async_trait]
impl Command for CreateLogEntry {
    type Output = ();

    #[tracing::instrument(skip(db))]
    async fn execute(self, db: &Database) -> Result<Self::Output> {
        tracing::debug!("Creating log entry");

        sqlx::query("INSERT INTO logs (event, payload) VALUES (?, ?)")
            .bind(&self.event)
            .bind(&self.payload)
            .execute(db.lock().await.deref_mut())
            .await
            .with_context(|| format!("Couldn't create log entry: {:?}", self))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Database, GetLogEntries};

    #[tokio::test(threaded_scheduler)]
    async fn test() {
        let db = Database::mock().await;

        db.execute(CreateLogEntry {
            event: "some-event-1".to_string(),
            payload: "some-payload-1".to_string(),
        })
        .await
        .unwrap();

        db.execute(CreateLogEntry {
            event: "some-event-2".to_string(),
            payload: "some-payload-2".to_string(),
        })
        .await
        .unwrap();

        let logs = db.find_all(GetLogEntries).await.unwrap();

        assert_eq!(2, logs.len());
        assert_eq!("some-event-1", logs[0].event);
        assert_eq!("some-payload-1", logs[0].payload);
        assert_eq!("some-event-2", logs[1].event);
        assert_eq!("some-payload-2", logs[1].payload);
    }
}
