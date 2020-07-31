pub use self::{config::*, models::*};

use anyhow::{Context, Result};
use reqwest::header::HeaderMap;
use reqwest::{header, Client, Url};
use std::iter::FromIterator;
use std::time::Duration;

mod config;
mod endpoints;
mod models;

pub struct GitLabClient {
    url: Url,
    client: Client,
}

impl GitLabClient {
    pub fn new(config: GitLabClientConfig) -> Result<Self> {
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
        Self::new(GitLabClientConfig {
            url: format!("{}/gitlab", mockito::server_url()).parse().unwrap(),
            personal_access_token: "token".into(),
        })
        .unwrap()
    }

    pub async fn init(config: GitLabClientConfig) -> Result<Self> {
        let url = config.url.clone();
        let gitlab = Self::new(config)?;

        gitlab
            .ping()
            .await
            .with_context(|| format!("Couldn't ping GitLab at {}", url))?;

        Ok(gitlab)
    }
}
