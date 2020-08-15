use crate::features::prelude::*;
use crate::Reminder;

#[derive(Clone, Debug, Default)]
pub struct FindReminders {
    /// When set, returns reminder with specified id
    pub id: Option<Id<Reminder>>,

    /// When set, returns reminders overdue by given date (i.e. the ones where
    /// `reminders.remind_at <= $remind_at`)
    pub overdue_by: Option<DateTime<Utc>>,
}

impl FindReminders {
    pub fn id(id: Id<Reminder>) -> Self {
        Self {
            id: Some(id),
            ..Default::default()
        }
    }

    pub fn overdue_by(overdue_by: DateTime<Utc>) -> Self {
        Self {
            overdue_by: Some(overdue_by),
            ..Default::default()
        }
    }
}

#[async_trait]
impl Query for FindReminders {
    type Model = Reminder;

    #[tracing::instrument(skip(db))]
    async fn execute(self, db: &Database) -> Result<Vec<Self::Model>> {
        tracing::debug!("Finding reminders");

        let mut query = String::from("SELECT * FROM reminders WHERE 1 = 1");
        let mut args = SqliteArguments::default();

        if let Some(id) = self.id {
            query += " AND id = ?";
            args.add(id);
        }

        if let Some(overdue_by) = self.overdue_by {
            query += " AND remind_at <= ?";
            args.add(overdue_by);
        }

        sqlx::query_as_with(&query, args)
            .fetch_all(db.lock().await.deref_mut())
            .await
            .with_context(|| format!("Couldn't find reminders for query: {:?}", self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{create_merge_request, create_project, create_user};
    use crate::CreateReminder;
    use chrono::TimeZone;
    use std::collections::BTreeSet;

    struct TestContext {
        db: Database,
        reminders: [Id<Reminder>; 2],
    }

    impl TestContext {
        async fn new() -> Self {
            let db = Database::mock().await;

            let projects = [create_project(&db, 1).await, create_project(&db, 2).await];

            let merge_requests = [
                create_merge_request(&db, projects[0], 1, 1).await,
                create_merge_request(&db, projects[1], 2, 1).await,
            ];

            let users = [create_user(&db, 1).await, create_user(&db, 2).await];

            let reminder_1 = db
                .execute(CreateReminder {
                    user_id: users[0],
                    merge_request_id: merge_requests[0],
                    ext_discussion_id: gl::DiscussionId::new("cafebabe"),
                    message: None,
                    remind_at: Utc.timestamp(100, 0),
                })
                .await
                .unwrap();

            let reminder_2 = db
                .execute(CreateReminder {
                    user_id: users[1],
                    merge_request_id: merge_requests[1],
                    ext_discussion_id: gl::DiscussionId::new("cafebabe"),
                    message: Some("oh hi, mark!".to_string()),
                    remind_at: Utc.timestamp(200, 0),
                })
                .await
                .unwrap();

            Self {
                db,
                reminders: [reminder_1, reminder_2],
            }
        }

        async fn assert_query_returns(
            &self,
            query: FindReminders,
            expected: &[Id<Reminder>],
        ) -> Result<()> {
            let actual: BTreeSet<_> = self
                .db
                .get_all(query)
                .await?
                .into_iter()
                .map(|reminder| reminder.id)
                .collect();

            let expected: BTreeSet<_> = expected.iter().cloned().collect();

            if actual == expected {
                Ok(())
            } else {
                bail!(
                    "Query returned different result set:\n- actual={:?}\n- expected={:?}",
                    actual,
                    expected
                )
            }
        }
    }

    mod given_empty_filter {
        use super::*;

        #[tokio::test(threaded_scheduler)]
        async fn returns_all_items() {
            let ctxt = TestContext::new().await;
            let query = FindReminders::default();

            ctxt.assert_query_returns(query, &ctxt.reminders)
                .await
                .unwrap();
        }
    }

    mod given_filter_with_id {
        use super::*;

        #[tokio::test(threaded_scheduler)]
        async fn returns_reminder_with_given_id() {
            let ctxt = TestContext::new().await;

            let cases = vec![
                (ctxt.reminders[0], vec![ctxt.reminders[0]]),
                (ctxt.reminders[1], vec![ctxt.reminders[1]]),
            ];

            for (case_idx, (id, expected)) in cases.into_iter().enumerate() {
                let query = FindReminders::id(id);

                ctxt.assert_query_returns(query, &expected)
                    .await
                    .with_context(|| format!("Test case #{} failed", case_idx))
                    .unwrap();
            }
        }
    }

    mod given_filter_with_overdue_by {
        use super::*;

        #[tokio::test(threaded_scheduler)]
        async fn returns_reminders_overdue_by_given_datetime() {
            let ctxt = TestContext::new().await;

            let cases = vec![
                (Utc.timestamp(0, 0), vec![]),
                //
                (Utc.timestamp(100, 0), vec![ctxt.reminders[0]]),
                //
                (
                    Utc.timestamp(200, 0),
                    vec![ctxt.reminders[0], ctxt.reminders[1]],
                ),
                //
                (
                    Utc.timestamp(1000, 0),
                    vec![ctxt.reminders[0], ctxt.reminders[1]],
                ),
            ];

            for (case_idx, (overdue_by, expected)) in cases.into_iter().enumerate() {
                let query = FindReminders::overdue_by(overdue_by);

                ctxt.assert_query_returns(query, &expected)
                    .await
                    .with_context(|| format!("Test case #{} failed", case_idx))
                    .unwrap();
            }
        }
    }
}
