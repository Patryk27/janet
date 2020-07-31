pub use self::config::*;

use crate::gitlab::GitLabWebhookHandler;
use std::sync::Arc;
use warp::Filter;

mod config;
mod endpoints;

pub async fn init(config: HttpConfig, gitlab_webhook_handler: Arc<GitLabWebhookHandler>) {
    log::trace!("init()");

    let router = endpoints::health().or(endpoints::handle_gitlab_webhook(gitlab_webhook_handler));

    log::info!("Starting server at: {}", config.addr);

    warp::serve(router).run(config.addr).await;
}
