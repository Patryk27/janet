use anyhow::*;
use lib_database::{self as db, Database};
use lib_gitlab::{self as gl, GitLabClient};
use lib_interface::{MergeRequestPtr, PtrContext};

/// Loads user from GitLab and upserts it into the database.
pub async fn sync_user(
    db: &Database,
    gitlab: &GitLabClient,
    gl_user_id: gl::UserId,
) -> Result<(gl::User, db::Id<db::User>)> {
    let gl_user = gitlab.user(gl_user_id).await?;

    let user_id = db.execute(db::CreateUser { ext_id: gl_user.id }).await?;

    Ok((gl_user, user_id))
}

/// Loads project from GitLab and upserts it into the database.
pub async fn sync_project(
    db: &Database,
    gitlab: &GitLabClient,
    gl_project_id: gl::ProjectId,
) -> Result<(gl::Project, db::Id<db::Project>)> {
    let gl_project = gitlab.project(&gl_project_id.inner().to_string()).await?;

    let project_id = db
        .execute(db::CreateProject {
            ext_id: gl_project.id,
        })
        .await?;

    Ok((gl_project, project_id))
}

/// Loads project & merge request from GitLab and upserts both into the
/// database.
pub async fn sync_merge_request(
    db: &Database,
    gitlab: &GitLabClient,
    gl_project_id: gl::ProjectId,
    gl_merge_request_iid: gl::MergeRequestIid,
) -> Result<(gl::Project, gl::MergeRequest, db::Id<db::MergeRequest>)> {
    let (gl_project, project_id) = sync_project(db, gitlab, gl_project_id).await?;

    let gl_merge_request = gitlab
        .merge_request(gl_project_id, gl_merge_request_iid)
        .await?;

    let merge_request_id = db
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
    db: &Database,
    gitlab: &GitLabClient,
    merge_request: &MergeRequestPtr,
    ptr_context: &PtrContext,
) -> Result<(gl::Project, gl::MergeRequest, db::Id<db::MergeRequest>)> {
    let (gl_project_id, gl_merge_request_iid) = merge_request.resolve(&gitlab, ptr_context).await?;

    sync_merge_request(db, gitlab, gl_project_id, gl_merge_request_iid).await
}
