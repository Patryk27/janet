use anyhow::Result;
use lib_gitlab::{
    GitLabClient,
    WebhookEvent,
    WebhookMergeRequest,
    WebhookMergeRequestAttrs,
    WebhookNoteAttrs,
    WebhookProject,
};
use lib_interface::{
    Event,
    MergeRequestCommand,
    MergeRequestCommandContext,
    MergeRequestPtr,
    ProjectPtr,
};
use lib_system::System;
use std::sync::Arc;

pub struct GitLabWebhookHandler {
    bot_name: String,
    // TODO
    webhook_secret: String,
    gitlab: Arc<GitLabClient>,
    system: Arc<System>,
}

impl GitLabWebhookHandler {
    pub fn new(
        bot_name: String,
        webhook_secret: String,
        gitlab: Arc<GitLabClient>,
        system: Arc<System>,
    ) -> Self {
        Self {
            bot_name,
            webhook_secret,
            gitlab,
            system,
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
        let project = project.id;
        let merge_request = object_attributes.iid;

        let evt = match object_attributes.action.as_str() {
            "close" => Some(Event::MergeRequestClosed {
                project,
                merge_request,
            }),

            "merge" => Some(Event::MergeRequestMerged {
                project,
                merge_request,
            }),

            "reopen" => Some(Event::MergeRequestReopened {
                project,
                merge_request,
            }),

            _ => None,
        };

        if let Some(evt) = evt {
            self.system.process_event(evt).await;
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

        let ctxt = MergeRequestCommandContext {
            user: object_attributes.author_id,
            merge_request: MergeRequestPtr::Iid {
                project: Some(ProjectPtr::Id(project.id)),
                merge_request: merge_request.iid,
            },
            discussion: object_attributes.discussion_id.clone(),
        };

        match MergeRequestCommand::parse(ctxt, cmd) {
            Ok(cmd) => {
                self.system.process_command(cmd).await;
            }

            Err(err) => {
                tracing::warn!(
                    "Couldn't parse command `{}`; the underlying error was: {}",
                    cmd,
                    err
                );

                let _: Result<()> = try {
                    let username = self
                        .gitlab
                        .user(object_attributes.author_id)
                        .await?
                        .username;

                    self.gitlab
                        .create_merge_request_note(
                            project.id,
                            merge_request.iid,
                            &object_attributes.discussion_id,
                            format!("@{}: sorry, I'm not sure what you mean - could you please remove your comment and re-send it?", username),
                        )
                        .await?;
                };
            }
        }
    }
}
