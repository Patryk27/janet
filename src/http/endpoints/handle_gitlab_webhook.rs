use crate::gitlab::GitLabWebhookHandler;
use bytes::Bytes;
use reqwest::StatusCode;
use std::sync::Arc;
use warp::filters::body;
use warp::{Filter, Rejection, Reply};

pub fn handle_gitlab_webhook(
    webhook_handler: Arc<GitLabWebhookHandler>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("webhooks" / "gitlab")
        .and(warp::any().map(move || webhook_handler.clone()))
        .and(body::content_length_limit(1 * 1024 * 1024))
        .and(body::bytes())
        .map(handle)
}

fn handle(webhook_handler: Arc<GitLabWebhookHandler>, body: Bytes) -> impl Reply {
    std::fs::write("/tmp/event.json", &body).unwrap();

    match serde_json::from_slice(&body) {
        Ok(event) => {
            webhook_handler.handle(event);
        }

        Err(error) => {
            let body = String::from_utf8(body.to_vec())
                .unwrap_or_else(|_| String::from("(not a valid UTF-8 string)"));

            log::warn!("Unknown event: {}", body);
            log::warn!("... serde said: {}", error);
        }
    }

    StatusCode::NO_CONTENT
}
