use crate::gitlab::GitLabWebhookHandler;
use bytes::Bytes;
use reqwest::StatusCode;
use std::sync::Arc;
use warp::filters::body;
use warp::{Filter, Rejection, Reply};

pub fn gitlab_webhook(
    handler: Arc<GitLabWebhookHandler>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("webhooks" / "gitlab")
        .and(warp::any().map(move || handler.clone()))
        .and(body::content_length_limit(5 * 1024 * 1024))
        .and(body::bytes())
        .and_then(handle)
}

async fn handle(handler: Arc<GitLabWebhookHandler>, body: Bytes) -> Result<impl Reply, Rejection> {
    match serde_json::from_slice(&body) {
        Ok(event) => {
            handler.handle(event).await;
        }

        Err(error) => {
            let body = String::from_utf8(body.to_vec())
                .unwrap_or_else(|_| String::from("(not a valid UTF-8 string)"));

            tracing::warn!("Unknown event: {}", body);
            tracing::warn!("... serde said: {}", error);
        }
    }

    Ok(StatusCode::NO_CONTENT)
}
