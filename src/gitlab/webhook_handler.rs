use crate::cpu::Cpu;
use crate::gitlab::{
    WebhookEvent,
    WebhookMergeRequest,
    WebhookMergeRequestAttrs,
    WebhookNoteAttrs,
    WebhookProject,
};
use crate::interface::{Command, Event, MergeRequestPtr, ProjectPtr};
use std::sync::Arc;

pub struct GitLabWebhookHandler {
    bot_name: String,

    // TODO
    webhook_secret: String,

    cpu: Arc<Cpu>,
}

impl GitLabWebhookHandler {
    pub fn new(bot_name: String, webhook_secret: String, cpu: Arc<Cpu>) -> Self {
        Self {
            bot_name,
            webhook_secret,
            cpu,
        }
    }

    pub fn handle(&self, event: WebhookEvent) {
        log::debug!("Handling event: {:?}", event);

        match event {
            WebhookEvent::MergeRequest {
                project,
                object_attributes,
            } => self.handle_merge_request(project, object_attributes),

            WebhookEvent::Note {
                object_attributes,
                project,
                merge_request,
            } => self.handle_note(project, merge_request, object_attributes),
        }
    }

    fn handle_merge_request(
        &self,
        project: WebhookProject,
        object_attributes: WebhookMergeRequestAttrs,
    ) {
        let merge_request = MergeRequestPtr::Iid {
            project: Some(ProjectPtr::Id(project.id)),
            merge_request: object_attributes.iid,
        };

        let event = match object_attributes.action.as_str() {
            "close" => Some(Event::MergeRequestClosed(merge_request)),
            "merge" => Some(Event::MergeRequestMerged(merge_request)),
            "reopen" => todo!(),
            _ => None,
        };

        if let Some(event) = event {
            self.cpu.handle_event(event);
        }
    }

    fn handle_note(
        &self,
        project: WebhookProject,
        merge_request: WebhookMergeRequest,
        object_attributes: WebhookNoteAttrs,
    ) {
        let cmd = object_attributes.description;

        // TODO hard-coded janet
        if cmd.starts_with("@janet ") {
            let user = object_attributes.author_id;

            let merge_request = MergeRequestPtr::Iid {
                project: Some(ProjectPtr::Id(project.id)),
                merge_request: merge_request.iid,
            };

            let cmd = &cmd[7..]; // TODO

            match Command::parse(user, merge_request, cmd) {
                Ok(cmd) => {
                    self.cpu.handle_command(cmd);
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
}
