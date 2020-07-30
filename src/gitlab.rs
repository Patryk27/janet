pub use self::{config::*, models::*};

use anyhow::{Context, Result};
use reqwest::header::HeaderMap;
use reqwest::{header, Client, Url};
use serde::Serialize;
use std::iter::FromIterator;
use std::time::Duration;

mod config;
mod models;

pub struct GitLabClient {
    url: Url,
    client: Client,
}

impl GitLabClient {
    pub fn new(config: GitLabConfig) -> Result<Self> {
        let url = config.url.clone();

        let headers = HeaderMap::from_iter(vec![(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {}", config.personal_access_token))?,
        )]);

        let client = Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(15))
            .build()?;

        Ok(Self { url, client })
    }

    #[cfg(test)]
    pub fn mock() -> Self {
        Self::new(GitLabConfig {
            url: format!("{}/gitlab", mockito::server_url()).parse().unwrap(),
            personal_access_token: "token".into(),
            webhook_secret: "webhook_secret".into(),
        })
        .unwrap()
    }

    pub async fn init(config: GitLabConfig) -> Result<Self> {
        let url = config.url.clone();
        let gitlab = Self::new(config)?;

        gitlab
            .ping()
            .await
            .with_context(|| format!("Couldn't ping GitLab at {}", url))?;

        Ok(gitlab)
    }

    pub async fn ping(&self) -> Result<()> {
        log::trace!("ping()");

        self.client
            .get(self.url.clone())
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    pub async fn namespace(&self, id: impl AsRef<str>) -> Result<Namespace> {
        let id = id.as_ref().replace("/", "%2f");

        log::trace!("namespace(); id={}", id);

        let url = self
            .url
            .join("api/")?
            .join("v4/")?
            .join("namespaces/")?
            .join(&id)?;

        self.client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
            .map_err(Into::into)
    }

    pub async fn project(&self, id: impl AsRef<str>) -> Result<Project> {
        let id = id.as_ref().replace("/", "%2f");

        log::trace!("project(); id={}", id);

        let url = self
            .url
            .join("api/")?
            .join("v4/")?
            .join("projects/")?
            .join(&id)?;

        self.client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
            .map_err(Into::into)
    }

    pub async fn create_merge_request_note(
        &self,
        project_id: ProjectId,
        merge_request_iid: MergeRequestIid,
        note: impl AsRef<str>,
    ) -> Result<()> {
        let note = note.as_ref();

        log::trace!(
            "create_merge_request_note(); project_id={}, merge_request_iid={}, note={}",
            project_id.inner(),
            merge_request_iid.inner(),
            note
        );

        #[derive(Serialize)]
        struct Request {
            body: String,
        }

        let url = self
            .url
            .join("api/")?
            .join("v4/")?
            .join("projects/")?
            .join(&project_id.inner().to_string())?
            .join("merge_requests/")?
            .join(&merge_request_iid.inner().to_string())?
            .join("notes")?;

        let request = Request { body: note.into() };

        self.client
            .post(url)
            .json(&request)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}
