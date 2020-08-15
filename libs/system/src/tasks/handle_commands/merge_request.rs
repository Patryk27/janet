use crate::prelude::*;
use thiserror::Error;

mod hi;
mod manage_dependency;
mod manage_reminder;

pub type HandlerResult<T> = Result<T, HandlerError>;

#[derive(Debug, Error)]
pub enum HandlerError {
    #[error("sorry, I couldn't find this merge request - could you please ensure it exists and re-create your comment?")]
    MergeRequestNotFound,

    #[error("well, this is embarrassing - there was an issue processing your request:\n```\n{0:?}\n```\nCould you please contact the administrator?")]
    Unexpected(#[from] Error),
}

pub async fn handle(
    world: &World,
    ctxt: int::MergeRequestCommandContext,
    cmd: int::MergeRequestCommand,
) -> Result<()> {
    let result = match cmd {
        int::MergeRequestCommand::Hi => hi::handle(&world, &ctxt).await,

        int::MergeRequestCommand::ManageDependency { action, dependency } => {
            manage_dependency::handle(&world, &ctxt, action, dependency).await
        }

        int::MergeRequestCommand::ManageReminder { remind_at, message } => {
            manage_reminder::handle(&world, &ctxt, remind_at, message).await
        }
    };

    match result {
        Ok(()) => Ok(()),

        Err(err) => {
            let gl_user = world.gitlab.user(ctxt.user).await?;

            let (gl_project_id, gl_merge_request_iid) = ctxt
                .merge_request
                .resolve(&world.gitlab, &Default::default())
                .await?;

            world
                .gitlab
                .create_merge_request_note(
                    gl_project_id,
                    gl_merge_request_iid,
                    &ctxt.discussion,
                    format!("@{} {}", gl_user.username, err.to_string()),
                )
                .await?;

            if let HandlerError::Unexpected(err) = err {
                Err(err)
            } else {
                Ok(())
            }
        }
    }
}
