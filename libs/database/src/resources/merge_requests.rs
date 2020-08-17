pub use self::{merge_request::*, new_merge_request::*};

mod merge_request;
mod new_merge_request;

use crate::{Database, Id};
use anyhow::*;
use std::ops::DerefMut;

#[derive(Clone)]
pub struct MergeRequestsRepository {
    db: Database,
}

impl MergeRequestsRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    #[tracing::instrument(skip(self))]
    pub async fn add(&self, mr: &NewMergeRequest) -> Result<Id<MergeRequest>> {
        if let Some(id) = self.find_by_new(mr).await? {
            return Ok(id);
        }

        tracing::debug!("Accessing database");

        let mut conn = self.db.conn.lock().await;
        let id = Id::new();

        sqlx::query(
            "
            INSERT INTO merge_requests (
                id,
                project_id,
                ext_id,
                iid,
                state
            )
            VALUES (?, ?, ?, ?, ?)
            ",
        )
        .bind(id)
        .bind(mr.project_id)
        .bind(mr.ext_id)
        .bind(mr.iid)
        .bind(&mr.state)
        .execute(conn.deref_mut())
        .await
        .with_context(|| format!("Couldn't add merge request: {:?}", mr))?;

        Ok(id)
    }

    #[tracing::instrument(skip(self))]
    pub async fn get(&self, id: Id<MergeRequest>) -> Result<MergeRequest> {
        tracing::debug!("Accessing database");

        let mut conn = self.db.conn.lock().await;

        sqlx::query_as("SELECT * FROM merge_requests WHERE id = ?")
            .bind(id)
            .fetch_one(conn.deref_mut())
            .await
            .with_context(|| format!("Couldn't load merge request: {}", id))
    }

    #[tracing::instrument(skip(self))]
    pub async fn find_by_external_id(
        &self,
        project_ext_id: i64,
        iid: i64,
    ) -> Result<Option<Id<MergeRequest>>> {
        tracing::debug!("Accessing database");

        let mut conn = self.db.conn.lock().await;

        sqlx::query_as(
            "
            SELECT
                mr.id

            FROM
                merge_requests mr

            INNER JOIN
                projects p ON p.id = mr.project_id

            WHERE
                p.ext_id = ? AND
                mr.iid = ?
            ",
        )
        .bind(project_ext_id)
        .bind(iid)
        .fetch_optional(conn.deref_mut())
        .await
        .with_context(|| {
            format!(
                "Couldn't find merge request: project_ext_id={:?}, iid={}",
                project_ext_id, iid
            )
        })
    }

    #[tracing::instrument(skip(self))]
    async fn find_by_new(&self, mr: &NewMergeRequest) -> Result<Option<Id<MergeRequest>>> {
        tracing::debug!("Accessing database");

        let mut conn = self.db.conn.lock().await;

        sqlx::query_as(
            "
            SELECT
                id

            FROM
                merge_requests

            WHERE
                ext_id = ?
            ",
        )
        .bind(mr.ext_id)
        .fetch_optional(conn.deref_mut())
        .await
        .with_context(|| format!("Couldn't find merge request: {:?}", mr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod add {
        use super::*;
        use crate::NewProject;

        #[tokio::test(threaded_scheduler)]
        async fn test() {
            let db = Database::mock().await;

            let project_id = db.projects().add(&NewProject { ext_id: 1 }).await.unwrap();

            let mut ids = Vec::new();

            for i in 0..10 {
                let mr = NewMergeRequest {
                    project_id,
                    ext_id: i,
                    iid: i * 2,
                    state: "opened".to_string(),
                };

                let id = db.merge_requests().add(&mr).await.unwrap();
                let id2 = db.merge_requests().add(&mr).await.unwrap();

                assert_eq!(id2, id);

                ids.push(id);
            }

            for (i, id) in ids.into_iter().enumerate() {
                let i = i as i64;
                let mr = db.merge_requests().get(id).await.unwrap();

                assert_eq!(id, mr.id);
                assert_eq!(project_id, mr.project_id);
                assert_eq!(i, mr.ext_id);
                assert_eq!(i * 2, mr.iid);
                assert_eq!("opened", mr.state);
                assert_eq!(mr.checked_at, mr.created_at);
            }
        }
    }
}
