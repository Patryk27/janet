use crate::cpu::Cpu;
use crate::gitlab::{
    GitLabClient,
    WebhookEvent,
    WebhookMergeRequest,
    WebhookMergeRequestAttrs,
    WebhookNoteAttrs,
    WebhookProject,
};
use crate::interface::{Command, Event, MergeRequestPtr, ProjectPtr};
use anyhow::Result;
use std::sync::Arc;

pub struct GitLabWebhookHandler {
    bot_name: String,
    // TODO
    webhook_secret: String,
    gitlab: Arc<GitLabClient>,
    cpu: Arc<Cpu>,
}

impl GitLabWebhookHandler {
    pub fn new(
        bot_name: String,
        webhook_secret: String,
        gitlab: Arc<GitLabClient>,
        cpu: Arc<Cpu>,
    ) -> Self {
        Self {
            bot_name,
            webhook_secret,
            gitlab,
            cpu,
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn handle(&self, event: WebhookEvent) {
        tracing::debug!("Handling webhook event");

        match event {
            WebhookEvent::MergeRequest {
                project,
                object_attributes,
            } => self.handle_merge_request(project, object_attributes).await,

            WebhookEvent::Note {
                object_attributes,
                project,
                merge_request,
            } => {
                self.handle_note(project, merge_request, object_attributes)
                    .await
            }
        }
    }

    async fn handle_merge_request(
        &self,
        project: WebhookProject,
        object_attributes: WebhookMergeRequestAttrs,
    ) {
        let project_id = project.id;
        let merge_request_iid = object_attributes.iid;

        let event = match object_attributes.action.as_str() {
            "close" => Some(Event::MergeRequestClosed {
                project_id,
                merge_request_iid,
            }),

            "merge" => Some(Event::MergeRequestMerged {
                project_id,
                merge_request_iid,
            }),

            "reopen" => Some(Event::MergeRequestReopened {
                project_id,
                merge_request_iid,
            }),

            _ => None,
        };

        if let Some(event) = event {
            self.cpu.handle_event(event);
        }
    }

    #[tracing::instrument(skip(self))]
    async fn handle_note(
        &self,
        project: WebhookProject,
        merge_request: WebhookMergeRequest,
        object_attributes: WebhookNoteAttrs,
    ) {
        let cmd = object_attributes.description;

        // TODO hard-coded janet
        if !cmd.starts_with("@janet ") {
            return;
        }

        // TODO hard-coded 7
        let cmd = cmd[7..].trim();

        let user = object_attributes.author_id;
        let discussion = object_attributes.discussion_id;

        let merge_request_ptr = MergeRequestPtr::Iid {
            project: Some(ProjectPtr::Id(project.id)),
            merge_request: merge_request.iid,
        };

        match Command::parse(user, discussion.clone(), merge_request_ptr, cmd) {
            Ok(cmd) => {
                self.cpu.handle_command(cmd);
            }

            Err(err) => {
                tracing::warn!(
                    "Couldn't parse command `{}`; the underlying error was: {}",
                    cmd,
                    err
                );

                let _: Result<()> = try {
                    let username = self.gitlab.user(user).await?.username;

                    self
                        .gitlab
                        .create_merge_request_note(
                            project.id,
                            merge_request.iid,
                            &discussion,
                            format!("@{}: sorry, I'm not sure what you mean - could you please remove your comment and re-send it?", username),
                        )
                        .await?;
                };
            }
        }
    }
}
