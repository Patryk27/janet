use crate::database::DatabaseConfig;
use crate::gitlab::GitLabConfig;
use crate::http::HttpConfig;
use anyhow::{Context, Result};
use serde::Deserialize;
use tokio::fs;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub bot: BotConfig,
    pub database: DatabaseConfig,
    pub http: HttpConfig,
    pub gitlab: GitLabConfig,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BotConfig {
    pub name: String,
}

impl Config {
    pub async fn load() -> Result<Config> {
        let file = fs::read_to_string("config.toml")
            .await
            .context("Couldn't open file")?;

        toml::from_str(&file).context("Couldn't parse file")
    }
}
