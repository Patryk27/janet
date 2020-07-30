pub use self::{config::*, persistence::*};

use anyhow::{Context, Result};
use sqlx::{Connection, SqliteConnection};
use std::sync::Arc;
use tokio::sync::Mutex;

mod config;
mod migrations;
mod persistence;

#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<SqliteConnection>>,
}

impl Database {
    pub async fn new(config: DatabaseConfig) -> Result<Self> {
        let mut conn = SqliteConnection::connect(&config.path)
            .await
            .context("Couldn't initialize SQLite")?;

        migrations::run(&mut conn)
            .await
            .context("Couldn't migrate the database")?;

        let conn = Arc::new(Mutex::new(conn));

        Ok(Database { conn })
    }

    #[cfg(test)]
    pub async fn mock() -> Self {
        Self::new(DatabaseConfig {
            path: "sqlite::memory:".into(),
        })
        .await
        .unwrap()
    }

    #[cfg(test)]
    pub async fn lock(&self) -> tokio::sync::MutexGuard<'_, SqliteConnection> {
        self.conn.lock().await
    }

    pub fn logs(&self) -> LogsRepository {
        LogsRepository::new(self.clone())
    }

    pub fn merge_request_dependencies(&self) -> MergeRequestDependenciesRepository {
        MergeRequestDependenciesRepository::new(self.clone())
    }

    pub fn reminders(&self) -> RemindersRepository {
        RemindersRepository::new(self.clone())
    }
}
