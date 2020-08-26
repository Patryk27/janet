use crate::features::prelude::*;
use crate::{MergeRequest, Reminder, User};

#[derive(Clone, Debug)]
pub struct CreateReminder {
    /// Internal id of the user who should get reminded
    pub user_id: Id<User>,

    /// Internal id of the merge request where we should create comment
    pub merge_request_id: Id<MergeRequest>,

    /// GitLab's discussion id where we should create comment
    pub ext_discussion_id: gl::DiscussionId,

    /// Message to remind; optional
    pub message: Option<String>,

    /// When we should remind
    pub remind_at: DateTime<Utc>,
}

#[async_trait]
impl Command for CreateReminder {
    type Output = Id<Reminder>;

    #[tracing::instrument(skip(db))]
    async fn execute(self, db: &Database) -> Result<Self::Output> {
        tracing::debug!("Creating reminder");

        let id = Id::default();

        sqlx::query(
            "
            INSERT INTO reminders (
                id,
                user_id,
                merge_request_id,
                ext_discussion_id,
                message,
                remind_at
            )
            VALUES (?, ?, ?, ?, ?, ?)
            ",
        )
        .bind(id)
        .bind(self.user_id)
        .bind(self.merge_request_id)
        .bind(self.ext_discussion_id.as_ref())
        .bind(self.message.as_ref())
        .bind(self.remind_at)
        .execute(db.lock().await.deref_mut())
        .await
        .with_context(|| format!("Couldn't create reminder: {:?}", self))?;

        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{create_merge_request, create_project, create_user};
    use crate::FindReminders;

    async fn context() -> (Database, Id<User>, Id<MergeRequest>) {
        let db = Database::mock().await;
        let user_id = create_user(&db, 250).await;
        let project_id = create_project(&db, 10).await;
        let merge_request_id = create_merge_request(&db, project_id, 100, 1).await;

        (db, user_id, merge_request_id)
    }

    async fn test(db: &Database, command: CreateReminder) {
        let id = db.execute(command.clone()).await.unwrap();
        let reminder = db.get_one(FindReminders::id(id)).await.unwrap();

        assert_eq!(id, reminder.id);
        assert_eq!(command.user_id, reminder.user_id);
        assert_eq!(command.merge_request_id, reminder.merge_request_id);
        assert_eq!(
            command.ext_discussion_id.as_ref(),
            reminder.ext_discussion_id
        );
        assert_eq!(command.message, reminder.message);
        assert_eq!(command.remind_at, reminder.remind_at);
    }

    #[tokio::test(threaded_scheduler)]
    async fn with_message() {
        let (db, user_id, merge_request_id) = context().await;

        test(
            &db,
            CreateReminder {
                user_id,
                merge_request_id,
                ext_discussion_id: gl::DiscussionId::new("cafebabe"),
                message: Some("Hello, World!".to_string()),
                remind_at: Utc::now(),
            },
        )
        .await;
    }

    #[tokio::test(threaded_scheduler)]
    async fn without_message() {
        let (db, user_id, merge_request_id) = context().await;

        test(
            &db,
            CreateReminder {
                user_id,
                merge_request_id,
                ext_discussion_id: gl::DiscussionId::new("cafebabe"),
                message: None,
                remind_at: Utc::now(),
            },
        )
        .await;
    }
}
