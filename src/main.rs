#![feature(try_blocks)]
#![feature(type_ascription)]

use anyhow::{Context, Result};
use std::sync::Arc;

mod config;
mod cpu;
mod database;
mod gitlab;
mod http;
mod interface;
mod log;

const LOGO: &str = r#"
       __                 __ 
      / /___ _____  ___  / /_
 __  / / __ `/ __ \/ _ \/ __/
/ /_/ / /_/ / / / /  __/ /_  
\____/\__,_/_/ /_/\___/\__/  

"#;

#[tokio::main]
async fn main() -> Result<()> {
    let config = {
        config::Config::load()
            .await
            .context("Couldn't load configuration from `config.toml`")?
    };

    log::init(config.log).context("Couldn't initialize log")?;

    for line in LOGO.lines().skip(1) {
        tracing::info!("{}", line);
    }

    let db = {
        tracing::info!("Initializing database (path = {})", config.database.path);

        if config.database.path.contains(":memory:") {
            tracing::warn!("");
            tracing::warn!("!! STARTING WITH AN IN-MEMORY DATABASE !!");
            tracing::warn!("");
            tracing::warn!("When you restart Janet, she'll forget everything.");
            tracing::warn!(
                "To get rid of this warning, please change `database.path` to point at a file."
            );
            tracing::warn!("");
        }

        database::Database::new(config.database)
            .await
            .context("Couldn't initialize database")?
    };

    let gitlab = {
        tracing::info!("Initializing GitLab client");

        Arc::new(
            gitlab::GitLabClient::init(config.gitlab.client)
                .await
                .context("Couldn't initialize GitLab client")?,
        )
    };

    let cpu = {
        tracing::info!("Initializing CPU");

        Arc::new(cpu::Cpu::init(db, gitlab.clone()))
    };

    let gitlab_webhook_handler = Arc::new(gitlab::GitLabWebhookHandler::new(
        config.bot.name,
        config.gitlab.webhook_secret,
        gitlab.clone(),
        cpu.clone(),
    ));

    http::init(config.http, gitlab_webhook_handler).await;

    tracing::info!("Shutting down");

    Ok(())
}
