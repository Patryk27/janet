pub use self::{merge_request_dependency::*, new_merge_request_dependency::*};

use crate::database::{Database, Id, MergeRequestDependency, NewMergeRequestDependency};
use anyhow::*;
use chrono::{DateTime, Utc};
use std::ops::DerefMut;

mod merge_request_dependency;
mod new_merge_request_dependency;

#[derive(Clone)]
pub struct MergeRequestDependenciesRepository {
    db: Database,
}

impl MergeRequestDependenciesRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn add(&self, dep: &NewMergeRequestDependency) -> Result<Id<MergeRequestDependency>> {
        let mut conn = self.db.conn.lock().await;
        let id = Id::new();

        sqlx::query(
            "
            INSERT INTO merge_request_dependencies (
                id,
                user_id,
                source_project_id,
                source_merge_request_id,
                dependency_project_id,
                dependency_merge_request_id
            )
            VALUES (?, ?, ?, ?, ?)
            ",
        )
        .bind(id)
        .bind(dep.user_id)
        .bind(dep.source_project_id)
        .bind(dep.source_merge_request_id)
        .bind(dep.dependency_project_id)
        .bind(dep.dependency_merge_request_id)
        .execute(conn.deref_mut())
        .await
        .with_context(|| format!("Couldn't add merge request dependency: {:#?}", dep))?;

        Ok(id)
    }

    pub async fn touch_checked_at(&self, id: Id<MergeRequestDependency>) -> Result<()> {
        let mut conn = self.db.conn.lock().await;

        todo!()
    }

    pub async fn find_stale(&self, now: DateTime<Utc>) -> Result<Vec<MergeRequestDependency>> {
        let mut conn = self.db.conn.lock().await;

        sqlx::query_as("SELECT * FROM merge_request_dependencies WHERE checked_at >= ?")
            .bind(now)
            .fetch_all(conn.deref_mut())
            .await
            .context("Couldn't find stale merge request dependencies")
    }
}
