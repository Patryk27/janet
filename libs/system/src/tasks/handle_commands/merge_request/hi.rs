use super::HandlerResult;
use crate::prelude::*;

/// Handles the `hi` command
pub async fn handle(world: &World, ctxt: &int::MergeRequestCommandContext) -> HandlerResult<()> {
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
            format!("Hi, @{}!", gl_user.username),
        )
        .await?;

    Ok(())
}
