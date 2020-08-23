use crate::features::prelude::*;
use crate::{MergeRequest, MergeRequestDependency, User};

#[derive(Clone, Debug)]
pub struct CreateMergeRequestDependency {
    /// Internal id of the user who should be notified on change
    pub user_id: Id<User>,

    /// GitLab's discussion id
    pub ext_discussion_id: gl::DiscussionId,

    /// Internal id of the source merge request (i.e. the one where you write
    /// the `depends on` comment)
    pub src_merge_request_id: Id<MergeRequest>,

    /// Internal id of the destination merge request (i.e. the one referred
    /// inside the `depends on` comment)
    pub dst_merge_request_id: Id<MergeRequest>,
}

#[async_trait]
impl Command for CreateMergeRequestDependency {
    type Output = Id<MergeRequestDependency>;

    #[tracing::instrument(skip(db))]
    async fn execute(self, db: &Database) -> Result<Self::Output> {
        tracing::debug!("Creating merge request dependency");

        let id = Id::default();

        sqlx::query(
            "
            INSERT INTO merge_request_dependencies (
                id,
                user_id,
                ext_discussion_id,
                src_merge_request_id,
                dst_merge_request_id
            )
            VALUES (?, ?, ?, ?, ?)
            ",
        )
        .bind(id)
        .bind(self.user_id)
        .bind(self.ext_discussion_id.as_ref())
        .bind(self.src_merge_request_id)
        .bind(self.dst_merge_request_id)
        .execute(db.lock().await.deref_mut())
        .await
        .with_context(|| format!("Couldn't create merge request dependency: {:?}", self))?;

        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use crate::FindMergeRequestDependencies;

    #[tokio::test(threaded_scheduler)]
    async fn test() {
        let db = Database::mock().await;
        let user_id = create_user(&db, 250).await;
        let project_id = create_project(&db, 10).await;
        let src_merge_request_id = create_merge_request(&db, project_id, 100, 1).await;
        let dst_merge_request_id = create_merge_request(&db, project_id, 101, 2).await;

        let id = db
            .execute(CreateMergeRequestDependency {
                user_id,
                ext_discussion_id: gl::DiscussionId::new("cafebabe"),
                src_merge_request_id,
                dst_merge_request_id,
            })
            .await
            .unwrap();

        let dep = db
            .find_one(FindMergeRequestDependencies {
                id: Some(id),
                ..Default::default()
            })
            .await
            .unwrap();

        assert_eq!(id, dep.id);
        assert_eq!(user_id, dep.user_id);
        assert_eq!("cafebabe", dep.ext_discussion_id);
        assert_eq!(src_merge_request_id, dep.src_merge_request_id);
        assert_eq!(dst_merge_request_id, dep.dst_merge_request_id);
    }
}
