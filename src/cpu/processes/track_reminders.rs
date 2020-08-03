//! TODO unfinished feature: reminders

use crate::database::Database;
use anyhow::Result;
use tokio::time::{delay_for, Duration};

pub async fn track_reminders(db: Database) -> Result<()> {
    loop {
        delay_for(Duration::from_secs(5)).await;
    }
}
