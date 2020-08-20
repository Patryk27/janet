use crate::Database;
use anyhow::*;
use async_trait::async_trait;

#[async_trait]
pub trait Command {
    type Output;

    async fn execute(self, db: &Database) -> Result<Self::Output>;
}

#[async_trait]
pub trait Query {
    type Model;

    async fn execute(self, db: &Database) -> Result<Vec<Self::Model>>;
}
