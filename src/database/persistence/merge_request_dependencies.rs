pub use self::{merge_request_dependency::*, new_merge_request_dependency::*};

use crate::database::{Database, Id, MergeRequest, User};
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

    #[tracing::instrument(skip(self))]
    pub async fn add(&self, dep: &NewMergeRequestDependency) -> Result<Id<MergeRequestDependency>> {
        tracing::debug!("Accessing database");

        let mut conn = self.db.conn.lock().await;
        let id = Id::new();

        sqlx::query(
            "
            INSERT INTO merge_request_dependencies (
                id,
                user_id,
                discussion_ext_id,
                src_merge_request_id,
                dst_merge_request_id
            )
            VALUES (?, ?, ?, ?, ?)
            ",
        )
        .bind(id)
        .bind(dep.user_id)
        .bind(&dep.discussion_ext_id)
        .bind(dep.src_merge_request_id)
        .bind(dep.dst_merge_request_id)
        .execute(conn.deref_mut())
        .await
        .with_context(|| format!("Couldn't add merge request dependency: {:?}", dep))?;

        Ok(id)
    }

    #[tracing::instrument(skip(self))]
    pub async fn get(&self, id: Id<MergeRequestDependency>) -> Result<MergeRequestDependency> {
        tracing::debug!("Accessing database");

        let mut conn = self.db.conn.lock().await;

        sqlx::query_as("SELECT * FROM merge_request_dependencies WHERE id = ?")
            .bind(id)
            .fetch_one(conn.deref_mut())
            .await
            .with_context(|| format!("Couldn't load merge request dependency: {}", id))
    }

    #[tracing::instrument(skip(self))]
    pub async fn remove(&self, id: Id<MergeRequestDependency>) -> Result<()> {
        tracing::debug!("Accessing database");

        let mut conn = self.db.conn.lock().await;

        sqlx::query("DELETE FROM merge_request_dependencies WHERE id = ?")
            .bind(id)
            .execute(conn.deref_mut())
            .await
            .with_context(|| format!("Couldn't remove merge request dependency: {}", id))?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn find_by_src(
        &self,
        user_id: Id<User>,
        discussion_ext_id: &str,
        src_merge_request_id: Id<MergeRequest>,
    ) -> Result<Option<MergeRequestDependency>> {
        tracing::debug!("Accessing database");

        let mut conn = self.db.conn.lock().await;

        sqlx::query_as(
            "
            SELECT
                *
                
            FROM
                merge_request_dependencies
                
            WHERE
                user_id = ? AND
                discussion_ext_id = ? AND
                src_merge_request_id = ?
            ",
        )
        .bind(user_id)
        .bind(discussion_ext_id)
        .bind(src_merge_request_id)
        .fetch_optional(conn.deref_mut())
        .await
        .context("Couldn't find merge request dependency")
    }

    #[tracing::instrument(skip(self))]
    pub async fn find_by_dep(
        &self,
        dst_merge_request_id: Id<MergeRequest>,
    ) -> Result<Vec<MergeRequestDependency>> {
        tracing::debug!("Accessing database");

        let mut conn = self.db.conn.lock().await;

        sqlx::query_as(
            "
            SELECT
                *

            FROM
                merge_request_dependencies

            WHERE
                dst_merge_request_id = ?

            ORDER BY
                checked_at ASC
            ",
        )
        .bind(dst_merge_request_id)
        .fetch_all(conn.deref_mut())
        .await
        .context("Couldn't find depending merge request dependencies")
    }

    #[tracing::instrument(skip(self))]
    pub async fn find_stale(
        &self,
        checked_at: DateTime<Utc>,
    ) -> Result<Vec<MergeRequestDependency>> {
        tracing::debug!("Accessing database");

        let mut conn = self.db.conn.lock().await;

        sqlx::query_as(
            "
            SELECT
                *
                
            FROM
                merge_request_dependencies
                
            WHERE
                checked_at <= ?
                
            ORDER BY
                checked_at ASC",
        )
        .bind(checked_at)
        .fetch_all(conn.deref_mut())
        .await
        .context("Couldn't find stale merge request dependencies")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::{NewMergeRequest, NewProject, NewUser};

    mod add {
        use super::*;

        #[tokio::test(threaded_scheduler)]
        async fn test() {
            let db = Database::mock().await;

            let user_id = db.users().add(&NewUser { ext_id: 1 }).await.unwrap();

            let project_id = db
                .projects()
                .add(&NewProject { ext_id: 100 })
                .await
                .unwrap();

            let src_merge_request_id = db
                .merge_requests()
                .add(&NewMergeRequest {
                    project_id,
                    ext_id: 1234,
                    iid: 1,
                    state: "opened".to_string(),
                })
                .await
                .unwrap();

            let dst_merge_request_id = db
                .merge_requests()
                .add(&NewMergeRequest {
                    project_id,
                    ext_id: 1235,
                    iid: 2,
                    state: "opened".to_string(),
                })
                .await
                .unwrap();

            let id = db
                .merge_request_dependencies()
                .add(&NewMergeRequestDependency {
                    user_id,
                    discussion_ext_id: "CAFEBABE".to_string(),
                    src_merge_request_id,
                    dst_merge_request_id,
                })
                .await
                .unwrap();

            let dep = db.merge_request_dependencies().get(id).await.unwrap();

            assert_eq!(id, dep.id);
            assert_eq!(user_id, dep.user_id);
            assert_eq!("CAFEBABE", dep.discussion_ext_id);
            assert_eq!(src_merge_request_id, dep.src_merge_request_id);
            assert_eq!(dst_merge_request_id, dep.dst_merge_request_id);
            assert_eq!(dep.checked_at, dep.created_at);
        }
    }
}
