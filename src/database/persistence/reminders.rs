pub use self::{new_reminder::*, reminder::*};

use crate::database::{Database, Id};
use anyhow::*;
use sqlx::types::chrono::{DateTime, Utc};
use std::ops::DerefMut;

mod new_reminder;
mod reminder;

#[derive(Clone)]
pub struct RemindersRepository {
    db: Database,
}

impl RemindersRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn create(&self, reminder: &NewReminder) -> Result<Id<Reminder>> {
        let mut conn = self.db.conn.lock().await;
        let id = Id::new();

        sqlx::query(
            "
            INSERT INTO reminders (
                id,
                user_id,
                project_id,
                merge_request_iid,
                remind_at
            ) VALUES (?, ?, ?, ?, ?)
            ",
        )
        .bind(id)
        .bind(reminder.user_id)
        .bind(reminder.project_id)
        .bind(reminder.merge_request_iid)
        .bind(reminder.remind_at)
        .execute(conn.deref_mut())
        .await
        .with_context(|| format!("Couldn't create reminder: {:?}", reminder))?;

        Ok(id)
    }

    pub async fn delete(&self, id: Id<Reminder>) -> Result<()> {
        let mut conn = self.db.conn.lock().await;

        sqlx::query("DELETE FROM reminders WHERE id = ?")
            .bind(id)
            .execute(conn.deref_mut())
            .await
            .with_context(|| format!("Couldn't delete reminder: {:?}", id))?;

        Ok(())
    }

    pub async fn find_overdue(&self, now: DateTime<Utc>) -> Result<Vec<Reminder>> {
        let mut conn = self.db.conn.lock().await;

        sqlx::query_as("SELECT * FROM reminders WHERE remind_at >= ? ORDER BY remind_at ASC")
            .bind(now)
            .fetch_all(conn.deref_mut())
            .await
            .context("Couldn't find overdue reminders")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    async fn assert_reminder_count(db: &Database, expected: usize) {
        let actual = {
            let mut conn = db.lock().await;

            sqlx::query_as::<_, (i64,)>("SELECT count(*) FROM reminders")
                .fetch_one(conn.deref_mut())
                .await
                .unwrap()
                .0 as usize
        };

        assert_eq!(expected, actual);
    }

    mod create {
        use super::*;

        #[tokio::test(threaded_scheduler)]
        async fn allows_to_create_many_reminders_for_different_users_and_projects_and_merge_requests(
        ) {
            let db = Database::mock().await;

            db.reminders()
                .create(&NewReminder {
                    user_id: 1,
                    project_id: 2,
                    merge_request_iid: 3,
                    remind_at: Utc.ymd(2012, 1, 1).and_hms(0, 0, 0),
                })
                .await
                .unwrap();

            db.reminders()
                .create(&NewReminder {
                    user_id: 4,
                    project_id: 5,
                    merge_request_iid: 6,
                    remind_at: Utc.ymd(2012, 1, 1).and_hms(0, 0, 0),
                })
                .await
                .unwrap();

            assert_reminder_count(&db, 2).await;
        }

        #[tokio::test(threaded_scheduler)]
        async fn allows_to_create_many_reminders_for_same_user_and_project_and_merge_request() {
            let db = Database::mock().await;

            for hour in 1..6 {
                db.reminders()
                    .create(&NewReminder {
                        user_id: 1,
                        project_id: 2,
                        merge_request_iid: 3,
                        remind_at: Utc.ymd(2012, 1, 1).and_hms(hour, 0, 0),
                    })
                    .await
                    .unwrap();
            }

            assert_reminder_count(&db, 5).await;
        }
    }

    // TODO mod delete { }

    mod find_overdue {
        use super::*;

        #[tokio::test(threaded_scheduler)]
        async fn test() {
            let db = Database::mock().await;

            for i in 1..10 {
                db.reminders()
                    .create(&NewReminder {
                        user_id: i as _,
                        project_id: 2,
                        merge_request_iid: 3,
                        remind_at: Utc.ymd(2012, 1, i).and_hms(0, 0, 0),
                    })
                    .await
                    .unwrap();
            }

            assert_reminder_count(&db, 9).await;

            let reminders = db
                .reminders()
                .find_overdue(Utc.ymd(2012, 1, 7).and_hms(0, 0, 0))
                .await
                .unwrap();

            assert_eq!(3, reminders.len());

            assert_eq!(7, reminders[0].user_id);
            assert_eq!(Utc.ymd(2012, 1, 7).and_hms(0, 0, 0), reminders[0].remind_at);

            assert_eq!(8, reminders[1].user_id);
            assert_eq!(Utc.ymd(2012, 1, 8).and_hms(0, 0, 0), reminders[1].remind_at);

            assert_eq!(9, reminders[2].user_id);
            assert_eq!(Utc.ymd(2012, 1, 9).and_hms(0, 0, 0), reminders[2].remind_at);
        }
    }
}
