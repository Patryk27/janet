use crate::features::prelude::*;
use crate::Reminder;

#[derive(Clone, Debug)]
pub struct DeleteReminder {
    pub id: Id<Reminder>,
}

#[async_trait]
impl Command for DeleteReminder {
    type Output = ();

    #[tracing::instrument(skip(db))]
    async fn execute(self, db: &Database) -> Result<Self::Output> {
        tracing::debug!("Deleting reminder");

        sqlx::query("DELETE FROM reminders WHERE id = ?")
            .bind(self.id)
            .execute(db.lock().await.deref_mut())
            .await
            .with_context(|| format!("Couldn't delete reminder: {:?}", self))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{create_merge_request, create_project, create_user};
    use crate::{CreateReminder, FindReminders};

    pub async fn exists(db: &Database, id: Id<Reminder>) -> bool {
        db.get_one(FindReminders {
            id: Some(id),
            ..Default::default()
        })
        .await
        .is_ok()
    }

    #[tokio::test(threaded_scheduler)]
    async fn test() {
        let db = Database::mock().await;
        let user_id = create_user(&db, 250).await;
        let project_id = create_project(&db, 10).await;
        let merge_request_id = create_merge_request(&db, project_id, 100, 1).await;

        let mut ids = Vec::new();

        for i in 0..2 {
            let id = db
                .execute(CreateReminder {
                    user_id,
                    merge_request_id,
                    ext_discussion_id: gl::DiscussionId::new(format!("cafebabe-{}", i)),
                    message: None,
                    remind_at: Utc::now(),
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

        // Remove first reminder
        {
            db.execute(DeleteReminder { id: ids[0] }).await.unwrap();

            assert!(!exists(&db, ids[0]).await);
            assert!(exists(&db, ids[1]).await);
        }

        // Remove second reminder
        {
            db.execute(DeleteReminder { id: ids[1] }).await.unwrap();

            assert!(!exists(&db, ids[0]).await);
            assert!(!exists(&db, ids[1]).await);
        }
    }
}
