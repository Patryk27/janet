use super::HandlerResult;
use crate::SystemDeps;
use lib_interface::MergeRequestCommandContext;

/// Handles the `hi` command.
pub async fn handle(deps: &SystemDeps, ctxt: &MergeRequestCommandContext) -> HandlerResult<()> {
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
            format!("Hi, @{}!", gl_user.username),
        )
        .await?;

    Ok(())
}
