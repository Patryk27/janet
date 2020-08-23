#![feature(crate_visibility_modifier)]

pub use self::{config::*, cqrs::*, features::*, id::*};

mod config;
mod cqrs;
mod features;
mod id;
mod migrations;

use anyhow::*;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{ConnectOptions, SqliteConnection};
use std::sync::Arc;
use tokio::sync::Mutex;

#[cfg(test)]
mod test_utils;

#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<SqliteConnection>>,
}

impl Database {
    pub async fn new(config: DatabaseConfig) -> Result<Self> {
        if config.path.contains(":memory:") {
            tracing::warn!("");
            tracing::warn!("!! STARTING WITH AN IN-MEMORY DATABASE !!");
            tracing::warn!("");
            tracing::warn!("When you restart Janet, she'll forget everything.");
            tracing::warn!(
                "To get rid of this warning, please change `database.path` to point at a file."
            );
            tracing::warn!("");
        }

        let options = SqliteConnectOptions::new()
            .filename(&config.path)
            .foreign_keys(true)
            .statement_cache_capacity(0); // Statement cache is too overzealous and makes `DROP TABLE` statements fail

        let mut conn = options
            .connect()
            .await
            .context("Couldn't initialize SQLite")?;

        migrations::run(&mut conn)
            .await
            .context("Couldn't migrate the database")?;

        let conn = Arc::new(Mutex::new(conn));

        Ok(Database { conn })
    }

    pub async fn mock() -> Self {
        Self::new(DatabaseConfig {
            path: ":memory:".into(),
        })
        .await
        .unwrap()
    }

    pub async fn execute<C: Command>(&self, command: C) -> Result<C::Output> {
        command.execute(self).await
    }

    pub async fn get_all<Q: Query>(&self, query: Q) -> Result<Vec<Q::Model>> {
        query.execute(self).await
    }

    pub async fn get_one<Q: Query>(&self, query: Q) -> Result<Q::Model> {
        match self.get_opt(query).await? {
            Some(model) => Ok(model),
            None => bail!("No models match given query"),
        }
    }

    pub async fn get_opt<Q: Query>(&self, query: Q) -> Result<Option<Q::Model>> {
        let model = self.get_all(query).await?.into_iter().next();

        Ok(model)
    }

    crate async fn lock(&self) -> tokio::sync::MutexGuard<'_, SqliteConnection> {
        self.conn.lock().await
    }
}
