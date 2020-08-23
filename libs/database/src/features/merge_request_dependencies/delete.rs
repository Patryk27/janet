use crate::features::prelude::*;
use crate::MergeRequestDependency;

#[derive(Clone, Debug)]
pub struct DeleteMergeRequestDependency {
    pub id: Id<MergeRequestDependency>,
}

#[async_trait]
impl Command for DeleteMergeRequestDependency {
    type Output = ();

    #[tracing::instrument(skip(db))]
    async fn execute(self, db: &Database) -> Result<Self::Output> {
        tracing::debug!("Deleting merge request dependency");

        sqlx::query("DELETE FROM merge_request_dependencies WHERE id = ?")
            .bind(self.id)
            .execute(db.lock().await.deref_mut())
            .await
            .with_context(|| format!("Couldn't delete merge request dependency: {:?}", self))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{create_merge_request, create_project, create_user};
    use crate::{CreateMergeRequestDependency, GetMergeRequestDependencies};

    pub async fn exists(db: &Database, id: Id<MergeRequestDependency>) -> bool {
        db.find_one(GetMergeRequestDependencies {
            id: Some(id),
            ..Default::default()
        })
        .await
        .is_ok()
    }

    #[tokio::test(threaded_scheduler)]
    async fn test() {
        let db = Database::mock().await;
        let project_id = create_project(&db, 10).await;

        let mut ids = Vec::new();

        for i in 0..2 {
            let id = db
                .execute(CreateMergeRequestDependency {
                    user_id: create_user(&db, 250 + i).await,
                    ext_discussion_id: gl::DiscussionId::new("cafebabe"),
                    src_merge_request_id: create_merge_request(&db, project_id, 100, 1).await,
                    dst_merge_request_id: create_merge_request(&db, project_id, 101, 2).await,
                })
                .await
                .unwrap();

            ids.push(id);
        }

        // Initial state
        {
            assert!(exists(&db, ids[0]).await);
            assert!(exists(&db, ids[1]).await);
        }

        // Remove first dependency
        {
            db.execute(DeleteMergeRequestDependency { id: ids[0] })
                .await
                .unwrap();

            assert!(!exists(&db, ids[0]).await);
            assert!(exists(&db, ids[1]).await);
        }

        // Remove second dependency
        {
            db.execute(DeleteMergeRequestDependency { id: ids[1] })
                .await
                .unwrap();

            assert!(!exists(&db, ids[0]).await);
            assert!(!exists(&db, ids[1]).await);
        }
    }
}
