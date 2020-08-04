//! TODO

use crate::database::Database;
use anyhow::*;
use tokio::time::{delay_for, Duration};

pub async fn track_merge_requests(db: Database) -> Result<()> {
    loop {
        delay_for(Duration::from_secs(5)).await;
    }
}
