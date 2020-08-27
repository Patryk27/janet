use super::HandlerResult;
use crate::prelude::*;
use chrono::Local;

/// Handles the `remind me` command
pub async fn handle(
    world: &World,
    ctxt: &int::MergeRequestCommandContext,
    remind_at: int::DateTime,
    message: Option<String>,
) -> HandlerResult<()> {
    let remind_at = remind_at.resolve_utc(Local::now())?;

    let (gl_user, user_id) = sync_user(world, ctxt.user).await?;

    let (gl_project, gl_merge_request, merge_request_id) =
        sync_merge_request_ptr(world, &ctxt.merge_request, &Default::default()).await?;

    world
        .db
        .execute(db::CreateReminder {
            user_id,
            merge_request_id,
            ext_discussion_id: ctxt.discussion.clone(),
            message,
            remind_at,
        })
        .await?;

    // TODO maybe we could thumbs-up the post instead of sending a comment?

    world
        .gitlab
        .create_merge_request_note(
            gl_project.id,
            gl_merge_request.iid,
            &ctxt.discussion,
            format!("@{} :+1:", gl_user.username),
        )
        .await?;

    Ok(())
}
