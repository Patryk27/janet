use crate::framework::{Janet, CONFIG, TMP_DIR};
use anyhow::Context;
use lib_gitlab::mock::GitLabMockServer;
use lib_gitlab::GitLabClient;
use std::fmt;
use std::path::Path;
use tokio::fs;

pub struct TestContext {
    pub gitlab: GitLabMockServer,
    pub janet: Janet,
}

impl TestContext {
    pub async fn create() -> Self {
        // Remove possible artifacts from the previous test; since the directory might
        // not exist yet (e.g. if this is the first time we're running tests at this
        // machine), no need to `.unwrap()` here
        let _ = fs::remove_dir_all(TMP_DIR).await;

        fs::create_dir(TMP_DIR)
            .await
            .with_context(|| format!("Couldn't create temporary directory: {}", TMP_DIR))
            .unwrap();

        let config_path = format!("{}/config.toml", TMP_DIR);
        let database_path = format!("{}/database.db", TMP_DIR);

        let (gitlab, _) = GitLabClient::mock().await;

        let config = String::from(CONFIG)
            .replace("{{ database_path }}", &database_path)
            .replace("{{ gitlab_url }}", gitlab.url().as_str());

        fs::write(&config_path, config)
            .await
            .with_context(|| format!("Couldn't create configuration file: {}", config_path))
            .unwrap();

        fs::write(&database_path, "")
            .await
            .with_context(|| format!("Couldn't create database file: {}", database_path))
            .unwrap();

        let janet = Janet::start(Path::new(&config_path)).await;

        Self { gitlab, janet }
    }
}

impl fmt::Debug for TestContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "TestContext")
    }
}
