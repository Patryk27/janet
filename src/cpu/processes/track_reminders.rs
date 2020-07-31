use crate::database::Database;
use anyhow::Result;
use chrono::Utc;
use tokio::time::{delay_for, Duration};

pub async fn track_reminders(db: Database) -> Result<()> {
    let reminders = db.reminders();

    loop {
        for reminder in reminders.find_overdue(Utc::now()).await? {
            // TODO
            db.reminders().remove(reminder.id).await?;
        }

        delay_for(Duration::from_secs(5)).await;
    }
}
