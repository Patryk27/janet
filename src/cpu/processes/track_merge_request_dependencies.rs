//! TODO

use crate::database::Database;
use anyhow::Result;
use chrono::Utc;
use tokio::time::{delay_for, Duration};

pub async fn track_merge_request_dependencies(db: Database) -> Result<()> {
    let merge_request_deps = db.merge_request_dependencies();

    loop {
        let checked_at = Utc::now() - chrono::Duration::minutes(30);

        for dep in merge_request_deps.find_stale(checked_at).await? {
            // tracing::trace!({ dep = ?dep }, "Checking stale merge request
            // dependency");
            //
            // db.merge_request_dependencies()
            //     .touch_checked_at(dep.id)
            //     .await?;
        }

        delay_for(Duration::from_secs(5)).await;
    }
}
