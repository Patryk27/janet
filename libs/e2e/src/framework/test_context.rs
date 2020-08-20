use crate::framework::{Janet, CONFIG};
use anyhow::*;
use lib_gitlab::mock::GitLabMockServer;
use rand::Rng;
use std::fmt;
use std::path::{Path, PathBuf};
use tempdir::TempDir;
use tokio::fs;
use tokio::net::TcpListener;
use tokio::time::{delay_for, Duration};

pub struct TestContext {
    pub temp: TempDir,
    pub gitlab: GitLabMockServer,
    pub janet: Janet,
}

impl TestContext {
    pub async fn create() -> Result<Self> {
        // This is technically a blocking call, but we're inside tests, so we don't care
        // that much
        let temp = TempDir::new("janet-").context("Couldn't create temporary directory")?;

        let gitlab = GitLabMockServer::start().await;

        let (http_addr, http_addr_guard) = Self::reserve_socket().await?;

        let database_path = Self::create_database(temp.path()).await?;

        let config_path = Self::create_config(
            temp.path(),
            &database_path.display().to_string(),
            gitlab.url().as_str(),
            &http_addr,
        )
        .await?;

        drop(http_addr_guard);

        let janet = Janet::start(http_addr, config_path).await?;

        Ok(Self {
            temp,
            gitlab,
            janet,
        })
    }

    async fn reserve_socket() -> Result<(String, TcpListener)> {
        for _ in 0..10000 {
            let port: u16 = rand::thread_rng().gen_range(1025, 65535);

            if let Ok(listener) = TcpListener::bind(("127.0.0.1", port)).await {
                return Ok((listener.local_addr()?.to_string(), listener));
            }

            delay_for(Duration::from_millis(1)).await;
        }

        bail!("Couldn't find free TCP port")
    }

    async fn create_database(temp: &Path) -> Result<PathBuf> {
        let path = temp.join("database.db");

        fs::write(&path, "")
            .await
            .with_context(|| format!("Couldn't create database file: {}", path.display()))?;

        Ok(path)
    }

    async fn create_config(
        temp: &Path,
        database_path: &str,
        gitlab_url: &str,
        http_addr: &str,
    ) -> Result<PathBuf> {
        let path = temp.join("config.toml");

        let content = String::from(CONFIG)
            .replace("{{ database.path }}", database_path)
            .replace("{{ gitlab.url }}", gitlab_url)
            .replace("{{ http.addr }}", http_addr);

        fs::write(&path, content)
            .await
            .with_context(|| format!("Couldn't create configuration file: {}", path.display()))?;

        Ok(path)
    }
}

impl fmt::Debug for TestContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "TestContext")
    }
}
