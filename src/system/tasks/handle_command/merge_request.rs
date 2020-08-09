mod hi;
mod manage_dependency;

use crate::interface::{MergeRequestCommand, MergeRequestCommandContext};
use crate::system::SystemDeps;
use anyhow::*;
use thiserror::Error;

pub type HandlerResult<T> = Result<T, HandlerError>;

// TODO integrate with InterfaceError, maybe?
#[derive(Debug, Error)]
pub enum HandlerError {
    #[error("sorry, I couldn't find this merge request - could you please ensure it exists and re-create your comment?")]
    MergeRequestNotFound,

    #[error("well, this is embarrassing - there was an issue processing your request: {0}; could you please contact the administrator?")]
    Unhandled(#[from] Error),
}

pub async fn handle(
    deps: &SystemDeps,
    ctxt: MergeRequestCommandContext,
    cmd: MergeRequestCommand,
) -> Result<()> {
    let result = match cmd {
        MergeRequestCommand::Hi => hi::handle(&deps, &ctxt).await,

        MergeRequestCommand::ManageDependency { action, dependency } => {
            manage_dependency::handle(&deps, &ctxt, action, dependency).await
        }

        MergeRequestCommand::ManageReminder { .. } => todo!(),
    };

    match result {
        Ok(_) => Ok(()),

        Err(err) => {
            let gl_user = deps.gitlab.user(ctxt.user).await?;

            let (gl_project_id, gl_merge_request_iid) = ctxt
                .merge_request
                .resolve(&deps.gitlab, &Default::default())
                .await?;

            deps.gitlab
                .create_merge_request_note(
                    gl_project_id,
                    gl_merge_request_iid,
                    &ctxt.discussion,
                    format!("@{} {}", gl_user.username, err.to_string()),
                )
                .await?;

            if let HandlerError::Unhandled(err) = err {
                Err(err)
            } else {
                Ok(())
            }
        }
    }
}
