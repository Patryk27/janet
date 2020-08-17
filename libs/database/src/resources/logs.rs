pub use self::{log::*, new_log::*};

mod log;
mod new_log;

use crate::Database;
use anyhow::*;
use std::ops::DerefMut;

#[derive(Clone)]
pub struct LogsRepository {
    db: Database,
}

impl LogsRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    #[tracing::instrument(skip(self))]
    pub async fn add(&self, log: NewLog) -> Result<()> {
        tracing::debug!("Accessing database");

        let mut conn = self.db.conn.lock().await;

        sqlx::query("INSERT INTO logs (event, payload) VALUES (?, ?)")
            .bind(&log.event)
            .bind(&log.payload)
            .execute(conn.deref_mut())
            .await
            .with_context(|| format!("Couldn't add log: {:?}", log))?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn find_all(&self) -> Result<Vec<Log>> {
        tracing::debug!("Accessing database");

        let mut conn = self.db.conn.lock().await;

        sqlx::query_as("SELECT * FROM logs")
            .fetch_all(conn.deref_mut())
            .await
            .context("Couldn't find logs")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod add {
        use super::*;

        #[tokio::test(threaded_scheduler)]
        async fn test() {
            let db = Database::mock().await;

            db.logs()
                .add(NewLog {
                    event: "some-event-1".to_string(),
                    payload: "some-payload-1".to_string(),
                })
                .await
                .unwrap();

            db.logs()
                .add(NewLog {
                    event: "some-event-2".to_string(),
                    payload: "some-payload-2".to_string(),
                })
                .await
                .unwrap();

            let logs = db.logs().find_all().await.unwrap();

            assert_eq!(2, logs.len());
            assert_eq!("some-event-1", logs[0].event);
            assert_eq!("some-payload-1", logs[0].payload);
            assert_eq!("some-event-2", logs[1].event);
            assert_eq!("some-payload-2", logs[1].payload);
        }
    }
}
