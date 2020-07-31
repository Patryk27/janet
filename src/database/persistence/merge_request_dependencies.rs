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
        log::trace!("add(); dep={:?}", dep);

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
            VALUES (?, ?, ?, ?, ?, ?)
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
        log::trace!("touch_checked_at(); id={}", id);

        todo!()
    }

    pub async fn find_depending(
        &self,
        dep_project_id: i64,
        dep_merge_request_iid: i64,
    ) -> Result<Vec<MergeRequestDependency>> {
        log::trace!(
            "find_depending(); dep_project_id={}, dep_merge_request_iid={}",
            dep_project_id,
            dep_merge_request_iid
        );

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
        log::trace!("find_stale(); checked_at={:?}", checked_at);

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

    mod add {
        use super::*;

        #[tokio::test(threaded_scheduler)]
        async fn test() {
            let db = Database::mock().await;

            db.merge_request_dependencies()
                .add(&NewMergeRequestDependency {
                    user_id: 1,
                    source_project_id: 100,
                    source_merge_request_iid: 1,
                    dependency_project_id: 120,
                    dependency_merge_request_iid: 3,
                })
                .await
                .unwrap();
        }
    }

    mod find_depending {
        use super::*;

        #[tokio::test(threaded_scheduler)]
        async fn test() {
            let db = Database::mock().await;
            let repo = db.merge_request_dependencies();

            let dep_1 = db
                .merge_request_dependencies()
                .add(&NewMergeRequestDependency {
                    user_id: 1,
                    source_project_id: 100,
                    source_merge_request_iid: 1,
                    dependency_project_id: 120,
                    dependency_merge_request_iid: 3,
                })
                .await
                .unwrap();

            let dep_2 = db
                .merge_request_dependencies()
                .add(&NewMergeRequestDependency {
                    user_id: 1,
                    source_project_id: 100,
                    source_merge_request_iid: 2,
                    dependency_project_id: 120,
                    dependency_merge_request_iid: 3,
                })
                .await
                .unwrap();

            let dep_3 = db
                .merge_request_dependencies()
                .add(&NewMergeRequestDependency {
                    user_id: 1,
                    source_project_id: 110,
                    source_merge_request_iid: 1,
                    dependency_project_id: 130,
                    dependency_merge_request_iid: 1,
                })
                .await
                .unwrap();

            {
                let deps = db
                    .merge_request_dependencies()
                    .find_depending(120, 3)
                    .await
                    .unwrap();

                assert_eq!(2, deps.len());
                assert!(deps.iter().any(|dep| dep.id == dep_1));
                assert!(deps.iter().any(|dep| dep.id == dep_2));
            }

            {
                let deps = db
                    .merge_request_dependencies()
                    .find_depending(130, 1)
                    .await
                    .unwrap();

                assert_eq!(1, deps.len());
                assert_eq!(dep_3, deps[0].id);
            }

            {
                let deps = db
                    .merge_request_dependencies()
                    .find_depending(130, 2)
                    .await
                    .unwrap();

                assert_eq!(0, deps.len());
            }
        }
    }

    mod find_stale {
        // TODO
    }
}
