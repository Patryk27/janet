use crate::{DiscussionId, MergeRequest, MergeRequestIid, Namespace, Project, ProjectId, User};
use serde_json::json;
use url::Url;
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

pub struct GitLabMockServer {
    inner: MockServer,
}

impl GitLabMockServer {
    pub async fn start() -> Self {
        let inner = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&inner)
            .await;

        Self { inner }
    }

    pub fn inner(&self) -> &MockServer {
        &self.inner
    }

    pub fn url(&self) -> Url {
        self.inner.uri().parse().unwrap()
    }

    pub async fn expect_merge_request(&self, merge_request: &MergeRequest) {
        let url = format!(
            "/api/v4/projects/{}/merge_requests/{}",
            merge_request.project_id.inner(),
            merge_request.iid.inner()
        );

        let response = ResponseTemplate::new(200).set_body_json(merge_request);

        Mock::given(method("GET"))
            .and(path(url))
            .respond_with(response)
            .mount(&self.inner)
            .await;
    }

    pub async fn expect_merge_request_note_created(
        &self,
        project: ProjectId,
        merge_request: MergeRequestIid,
        discussion: &DiscussionId,
        note: impl AsRef<str>,
    ) {
        let url = format!(
            "/api/v4/projects/{}/merge_requests/{}/discussions/{}/notes",
            project.inner(),
            merge_request.inner(),
            discussion.as_ref(),
        );

        let body = json!({
            "body": note.as_ref(),
        });

        Mock::given(method("POST"))
            .and(path(url))
            .and(body_json(&body))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&self.inner)
            .await;
    }

    pub async fn expect_namespace(&self, namespace: &Namespace) {
        let url = format!("/api/v4/namespaces/{}", namespace.id.inner());
        let response = ResponseTemplate::new(200).set_body_json(namespace);

        Mock::given(method("GET"))
            .and(path(url))
            .respond_with(response)
            .mount(&self.inner)
            .await;
    }

    pub async fn expect_project(&self, project: &Project) {
        let url = format!("/api/v4/projects/{}", project.id.inner());
        let response = ResponseTemplate::new(200).set_body_json(project);

        Mock::given(method("GET"))
            .and(path(url))
            .respond_with(response)
            .mount(&self.inner)
            .await;
    }

    pub async fn expect_user(&self, user: &User) {
        let url = format!("/api/v4/users/{}", user.id.inner());
        let response = ResponseTemplate::new(200).set_body_json(user);

        Mock::given(method("GET"))
            .and(path(url))
            .respond_with(response)
            .mount(&self.inner)
            .await;
    }
}
