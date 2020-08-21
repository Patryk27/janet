use crate::*;
use lib_gitlab as gl;

pub async fn create_merge_request(
    db: &Database,
    project_id: Id<Project>,
    id: usize,
    iid: usize,
) -> Id<MergeRequest> {
    db.execute(CreateMergeRequest {
        project_id,
        ext_id: gl::MergeRequestId::new(id),
        ext_iid: gl::MergeRequestIid::new(iid),
        ext_state: "opened".to_string(),
    })
    .await
    .unwrap()
}

pub async fn create_project(db: &Database, ext_id: usize) -> Id<Project> {
    db.execute(CreateProject {
        ext_id: gl::ProjectId::new(ext_id),
    })
    .await
    .unwrap()
}

pub async fn create_user(db: &Database, ext_id: usize) -> Id<User> {
    db.execute(CreateUser {
        ext_id: gl::UserId::new(ext_id),
    })
    .await
    .unwrap()
}
