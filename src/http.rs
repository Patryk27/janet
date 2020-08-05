pub use self::config::*;

mod config;
mod endpoints;

use crate::gitlab::GitLabWebhookHandler;
use anyhow::*;
use std::sync::Arc;
use warp::Filter;

pub async fn init(
    config: HttpConfig,
    gitlab_webhook_handler: Arc<GitLabWebhookHandler>,
) -> Result<()> {
    let router = endpoints::health().or(endpoints::handle_gitlab_webhook(gitlab_webhook_handler));

    tracing::info!("Starting server at: {}", config.addr);

    warp::serve(router).run(config.addr).await;

    Ok(())
}
