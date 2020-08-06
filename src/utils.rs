use anyhow::*;
use std::future::Future;
use tokio::task;

/// Spawns future in the background and flattens resulting `JoinError`.
/// Makes using `try_join!` way nicer.
pub async fn spawn_future<T, F>(fut: F) -> Result<T>
where
    T: 'static,
    F: Future<Output = Result<T>> + Send + 'static,
    F::Output: Send + 'static,
{
    task::spawn(fut).await?
}

#[cfg(test)]
pub mod for_tests {
    use crate::gitlab as gl;

    pub fn to_json<T: serde::Serialize>(model: &T) -> String {
        serde_json::to_string(model).unwrap()
    }

    pub fn mock_merge_request(merge_request: &gl::MergeRequest) -> mockito::Mock {
        let url = format!(
            "/gitlab/api/v4/projects/{}/merge_requests/{}",
            merge_request.project_id.inner(),
            merge_request.iid.inner()
        );

        mockito::mock("GET", url.as_str())
            .with_body(to_json(merge_request))
            .create()
    }

    pub fn mock_note_created(
        project: gl::ProjectId,
        merge_request: gl::MergeRequestIid,
        discussion_id: &gl::DiscussionId,
        body: impl AsRef<str>,
    ) -> mockito::Mock {
        let url = format!(
            "/gitlab/api/v4/projects/{}/merge_requests/{}/discussions/{}/notes",
            project.inner(),
            merge_request.inner(),
            discussion_id.as_ref()
        );

        let body = format!(r#"{{"body":"{}"}}"#, body.as_ref());

        mockito::mock("POST", url.as_str())
            .match_body(body.as_str())
            .create()
    }

    pub fn mock_project(project: &gl::Project) -> mockito::Mock {
        let url = format!("/gitlab/api/v4/projects/{}", project.id.inner());

        mockito::mock("GET", url.as_str())
            .with_body(to_json(project))
            .create()
    }

    pub fn mock_user(user: &gl::User) -> mockito::Mock {
        let url = format!("/gitlab/api/v4/users/{}", user.id.inner());

        mockito::mock("GET", url.as_str())
            .with_body(to_json(user))
            .create()
    }

    pub fn mock_default_user() -> mockito::Mock {
        mock_user(&gl::User {
            id: gl::UserId::new(100),
            username: "someone".to_string(),
        })
    }
}
