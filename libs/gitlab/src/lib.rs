#![feature(async_closure)]
#![feature(try_blocks)]
#![feature(type_ascription)]

pub use self::{config::*, models::*};

mod config;
mod endpoints;
mod models;

#[cfg(feature = "mock")]
pub mod mock;

use anyhow::Result;
use reqwest::header::HeaderMap;
use reqwest::{header, Client, Url};
use std::iter::FromIterator;
use std::time::Duration;

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

    #[cfg(feature = "mock")]
    pub async fn mock() -> (mock::GitLabMockServer, Self) {
        let server = mock::GitLabMockServer::start().await;

        let client = Self::new(GitLabConfig {
            url: server.url(),
            personal_access_token: "token".into(),
        })
        .unwrap();

        (server, client)
    }

    pub async fn init(config: GitLabConfig) -> Result<Self> {
        let gitlab = Self::new(config)?;

        gitlab.ping().await?;

        Ok(gitlab)
    }
}
