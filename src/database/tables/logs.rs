pub use self::{log::*, new_log::*};

use crate::database::Database;
use anyhow::*;
use std::ops::DerefMut;

mod log;
mod new_log;

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

        todo!()
    }
}

// TODO tests
