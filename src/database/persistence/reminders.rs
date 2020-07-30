pub use self::{new_reminder::*, reminder::*};

use crate::database::{Database, Id, NewReminder, Reminder};
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
                merge_request_id,
                remind_at
            ) VALUES (?, ?, ?, ?, ?)
            ",
        )
        .bind(id)
        .bind(reminder.user_id)
        .bind(reminder.project_id)
        .bind(reminder.merge_request_id)
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

        sqlx::query_as("SELECT * FROM reminders WHERE remind_at >= ?")
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

    mod find_overdue {
        use super::*;

        #[tokio::test(threaded_scheduler)]
        async fn test() {
            let db = Database::in_memory().await;

            db.reminders()
                .create(&NewReminder {
                    user_id: 1,
                    project_id: 2,
                    merge_request_id: 3,
                    remind_at: Utc::now(),
                })
                .await
                .unwrap();

            let reminders = db
                .reminders()
                .find_overdue(Utc.ymd(2012, 1, 1).and_hms(0, 0, 0))
                .await
                .unwrap();

            panic!("{:#?}", reminders);
        }
    }
}
