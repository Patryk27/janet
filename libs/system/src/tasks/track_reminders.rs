mod close_reminder;

use self::close_reminder::close_reminder;
use crate::prelude::*;
use chrono::Utc;
use tokio::time::{delay_for, Duration};

/// Starts an eternal loop  that watches for overdue reminders and notifies
/// related users
pub async fn start(world: Arc<World>) -> Result<()> {
    loop {
        let overdue_reminders = world
            .db
            .get_all(db::FindReminders::overdue_by(Utc::now()))
            .await?;

        for reminder in overdue_reminders {
            let id = reminder.id;

            if let Err(err) = close_reminder(&world, reminder).await {
                // TODO if target discussion does not exist, get rid of the reminder
                tracing::error!({ id = ?id, err = ?err }, "Couldn't close reminder");
            }
        }

        // We could piggy-back on Tokio's `DelayQueue`, but polling is good enough in
        // practice (and way simpler to implement!)
        delay_for(Duration::from_secs(5)).await;
    }
}
