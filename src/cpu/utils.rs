//! TODO proof of concept - requires solid refactoring

use crate::database::{self as db, Database, NewUser};
use crate::gitlab::{self as gl, GitLabClient};
use crate::interface::{MergeRequestPtr, PtrContext};
use anyhow::*;

pub async fn sync_user(
    db: &Database,
    gitlab: &GitLabClient,
    gl_user_id: gl::UserId,
) -> Result<(gl::User, db::Id<db::User>)> {
    let gl_user = gitlab.user(gl_user_id).await?;

    let user_id = db
        .users()
        .add(&NewUser {
            ext_id: gl_user.id.inner() as _,
        })
        .await?;

    Ok((gl_user, user_id))
}

pub async fn sync_merge_request(
    db: &Database,
    gitlab: &GitLabClient,
    ptr: &MergeRequestPtr,
    ptr_context: &PtrContext,
) -> Result<(gl::Project, gl::MergeRequest, db::Id<db::MergeRequest>)> {
    let (gl_project_id, gl_merge_request_iid) = ptr.resolve(&gitlab, ptr_context).await?;

    let gl_project = gitlab.project(&gl_project_id.inner().to_string()).await?;

    let gl_merge_request = gitlab
        .merge_request(gl_project_id, gl_merge_request_iid)
        .await?;

    let project_id = db
        .projects()
        .add(&db::NewProject {
            ext_id: gl_project_id.inner() as _,
        })
        .await?;

    let merge_request_id = db
        .merge_requests()
        .add(&db::NewMergeRequest {
            project_id,
            ext_id: gl_merge_request.id.inner() as _,
            iid: gl_merge_request.iid.inner() as _,
            state: gl_merge_request.state.clone(),
        })
        .await?;

    Ok((gl_project, gl_merge_request, merge_request_id))
}
