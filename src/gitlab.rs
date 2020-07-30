pub use self::{config::*, models::*};

use anyhow::{Context, Result};
use reqwest::header::HeaderMap;
use reqwest::{header, Client, Url};
use std::iter::FromIterator;

mod config;
mod models;

pub struct GitLabClient {
    url: Url,
    client: Client,
}

impl GitLabClient {
    pub async fn new(config: GitLabConfig) -> Result<Self> {
        let url = config.url.clone();

        let headers = HeaderMap::from_iter(vec![(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&config.personal_access_token)?,
        )]);

        let client = Client::builder().default_headers(headers).build()?;

        let this = Self {
            url: url.clone(),
            client,
        };

        this.ping()
            .await
            .with_context(|| format!("Couldn't ping GitLab at {}", url))?;

        Ok(this)
    }

    pub async fn ping(&self) -> Result<()> {
        self.client
            .get(self.url.clone())
            .send()
            .await
            .map(drop)
            .map_err(Into::into)
    }
}
