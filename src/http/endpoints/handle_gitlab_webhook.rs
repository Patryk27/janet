use crate::cpu::Cpu;
use crate::gitlab::{MergeRequestPtr, ProjectPtr, WebhookEvent};
use crate::interface::Command;
use bytes::Bytes;
use reqwest::StatusCode;
use std::sync::Arc;
use warp::filters::body;
use warp::{Filter, Rejection, Reply};

pub fn handle_gitlab_webhook(
    cpu: Arc<Cpu>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("webhooks" / "gitlab")
        .and(warp::any().map(move || cpu.clone()))
        .and(body::content_length_limit(1 * 1024 * 1024))
        .and(body::bytes())
        .map(handle)
}

fn handle(cpu: Arc<Cpu>, body: Bytes) -> impl Reply {
    if let Ok(event) = serde_json::from_slice(&body) {
        handle_event(cpu, event);
    } else {
        if let Ok(event) = String::from_utf8(body.to_vec()) {
            log::warn!("Unknown event: {}", event);
        } else {
            log::error!("Unknown event: (body is not a valid UTF-8)");
        }
    }

    StatusCode::NO_CONTENT
}

fn handle_event(cpu: Arc<Cpu>, event: WebhookEvent) {
    log::info!("Handling event: {:#?}", event);

    match event {
        WebhookEvent::Note {
            object_attributes,
            project: Some(project),
            merge_request: Some(merge_request),
        } => {
            let cmd = object_attributes.description;

            // TODO hard-coded janet
            if cmd.starts_with("@janet ") {
                let user = object_attributes.author_id;

                let merge_request = MergeRequestPtr::Id {
                    project: ProjectPtr::Id(project.id),
                    merge_request: merge_request.id,
                };

                let cmd = &cmd[7..]; // TODO

                match Command::parse(user, merge_request, cmd) {
                    Ok(cmd) => {
                        cpu.handle_command(cmd);
                    }

                    Err(err) => {
                        log::error!(
                            "Couldn't parse command `{}`; the underlying error was: {}",
                            cmd,
                            err
                        );
                    }
                }
            }
        }

        _ => (),
    }
}
