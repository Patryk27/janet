use crate::features::prelude::*;
use crate::{GetMergeRequests, MergeRequest, Project};

#[derive(Clone, Debug)]
pub struct CreateMergeRequest {
    /// Internal id of the related project
    pub project_id: Id<Project>,

    /// GitLab's merge request id
    pub ext_id: gl::MergeRequestId,

    /// GitLab's merge request incremental id
    pub ext_iid: gl::MergeRequestIid,

    /// GitLab's merge request state (e.g. "opened" or "merged")
    pub ext_state: String,
}

#[async_trait]
impl Command for CreateMergeRequest {
    type Output = Id<MergeRequest>;

    #[tracing::instrument(skip(db))]
    async fn execute(self, db: &Database) -> Result<Self::Output, Error> {
        // Creating merge request is idempotent - i.e. creating the same merge request
        // for the second time is a no-op
        if let Some(merge_request) = db
            .maybe_find_one(GetMergeRequests {
                ext_id: Some(self.ext_id),
                ..Default::default()
            })
            .await?
        {
            return Ok(merge_request.id);
        }

        tracing::debug!("Creating merge request");

        let id = Id::default();

        sqlx::query(
            "
            INSERT INTO merge_requests (
                id,
                project_id,
                ext_id,
                ext_iid,
                ext_state
            )
            VALUES (?, ?, ?, ?, ?)
            ",
        )
        .bind(id)
        .bind(self.project_id)
        .bind(self.ext_id.inner() as i64)
        .bind(self.ext_iid.inner() as i64)
        .bind(&self.ext_state)
        .execute(db.lock().await.deref_mut())
        .await
        .with_context(|| format!("Couldn't create merge request: {:?}", self))?;

        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_project;

    #[tokio::test(threaded_scheduler)]
    async fn test() {
        let db = Database::mock().await;
        let project_id = create_project(&db, 123).await;

        let id = db
            .execute(CreateMergeRequest {
                project_id,
                ext_id: gl::MergeRequestId::new(10),
                ext_iid: gl::MergeRequestIid::new(1),
                ext_state: "opened".to_string(),
            })
            .await
            .unwrap();

        let merge_request = db
            .find_one(GetMergeRequests {
                id: Some(id),
                ..Default::default()
            })
            .await
            .unwrap();

        assert_eq!(id, merge_request.id);
        assert_eq!(project_id, merge_request.project_id);
        assert_eq!(10, merge_request.ext_id as usize);
        assert_eq!(1, merge_request.ext_iid as usize);
        assert_eq!("opened", merge_request.ext_state);
        assert_eq!(merge_request.checked_at, merge_request.created_at);
    }

    mod when_creating_the_same_merge_request_for_the_second_time {
        use super::*;

        #[tokio::test(threaded_scheduler)]
        async fn returns_already_existing_id() {
            let db = Database::mock().await;

            for i in 0..5 {
                let command = CreateMergeRequest {
                    project_id: create_project(&db, i).await,
                    ext_id: gl::MergeRequestId::new(i),
                    ext_iid: gl::MergeRequestIid::new(1),
                    ext_state: "opened".to_string(),
                };

                let id_1 = db.execute(command.clone()).await.unwrap();
                let id_2 = db.execute(command.clone()).await.unwrap();

                assert_eq!(id_1, id_2);
            }
        }
    }
}
