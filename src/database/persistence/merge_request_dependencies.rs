pub use self::{merge_request_dependency::*, new_merge_request_dependency::*};

use crate::database::{Database, Id};
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
                source_merge_request_iid,
                dependency_project_id,
                dependency_merge_request_iid
            )
            VALUES (?, ?, ?, ?, ?)
            ",
        )
        .bind(id)
        .bind(dep.user_id)
        .bind(dep.source_project_id)
        .bind(dep.source_merge_request_iid)
        .bind(dep.dependency_project_id)
        .bind(dep.dependency_merge_request_iid)
        .execute(conn.deref_mut())
        .await
        .with_context(|| format!("Couldn't add merge request dependency: {:?}", dep))?;

        Ok(id)
    }

    pub async fn touch_checked_at(&self, id: Id<MergeRequestDependency>) -> Result<()> {
        todo!()
    }

    pub async fn find_depending(
        &self,
        dep_project_id: i64,
        dep_merge_request_iid: i64,
    ) -> Result<Vec<MergeRequestDependency>> {
        let mut conn = self.db.conn.lock().await;

        sqlx::query_as(
            "
            SELECT
                *

            FROM
                merge_request_dependencies

            WHERE
                dependency_project_id = ? AND
                dependency_merge_request_iid = ?

            ORDER BY
                checked_at ASC
            ",
        )
        .bind(dep_project_id)
        .bind(dep_merge_request_iid)
        .fetch_all(conn.deref_mut())
        .await
        .context("Couldn't find depending merge request dependencies")
    }

    pub async fn find_stale(
        &self,
        checked_at: DateTime<Utc>,
    ) -> Result<Vec<MergeRequestDependency>> {
        let mut conn = self.db.conn.lock().await;

        sqlx::query_as("SELECT * FROM merge_request_dependencies WHERE checked_at <= ? ORDER BY checked_at ASC")
            .bind(checked_at)
            .fetch_all(conn.deref_mut())
            .await
            .context("Couldn't find stale merge request dependencies")
    }
}
