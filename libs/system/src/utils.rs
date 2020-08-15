use crate::prelude::*;

/// Loads user from GitLab and upserts it into the database.
pub async fn sync_user(
    world: &World,
    gl_user_id: gl::UserId,
) -> Result<(gl::User, db::Id<db::User>)> {
    let gl_user = world.gitlab.user(gl_user_id).await?;

    let user_id = world
        .db
        .execute(db::CreateUser { ext_id: gl_user.id })
        .await?;

    Ok((gl_user, user_id))
}

/// Loads project from GitLab and upserts it into the database.
pub async fn sync_project(
    world: &World,
    gl_project_id: gl::ProjectId,
) -> Result<(gl::Project, db::Id<db::Project>)> {
    let gl_project = world
        .gitlab
        .project(&gl_project_id.inner().to_string())
        .await?;

    let project_id = world
        .db
        .execute(db::CreateProject {
            ext_id: gl_project.id,
        })
        .await?;

    Ok((gl_project, project_id))
}

/// Loads project & merge request from GitLab and upserts both into the
/// database.
pub async fn sync_merge_request(
    world: &World,
    gl_project_id: gl::ProjectId,
    gl_merge_request_iid: gl::MergeRequestIid,
) -> Result<(gl::Project, gl::MergeRequest, db::Id<db::MergeRequest>)> {
    let (gl_project, project_id) = sync_project(world, gl_project_id).await?;

    let gl_merge_request = world
        .gitlab
        .merge_request(gl_project_id, gl_merge_request_iid)
        .await?;

    let merge_request_id = world
        .db
        .execute(db::CreateMergeRequest {
            project_id,
            ext_id: gl_merge_request.id,
            ext_iid: gl_merge_request.iid,
            ext_state: gl_merge_request.state.clone(),
        })
        .await?;

    Ok((gl_project, gl_merge_request, merge_request_id))
}

/// Resolves pointer to merge request and upserts it into the database.
pub async fn sync_merge_request_ptr(
    world: &World,
    merge_request: &int::MergeRequestPtr,
    ptr_context: &int::PtrContext,
) -> Result<(gl::Project, gl::MergeRequest, db::Id<db::MergeRequest>)> {
    let (gl_project_id, gl_merge_request_iid) =
        merge_request.resolve(&world.gitlab, ptr_context).await?;

    sync_merge_request(world, gl_project_id, gl_merge_request_iid).await
}
