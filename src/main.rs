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
mod logging;

const LOGO: &str = r#"
       __                 __ 
      / /___ _____  ___  / /_
 __  / / __ `/ __ \/ _ \/ __/
/ /_/ / /_/ / / / /  __/ /_  
\____/\__,_/_/ /_/\___/\__/  

"#;

#[tokio::main]
async fn main() -> Result<()> {
    logging::init().context("Couldn't initialize logger")?;

    for line in LOGO.lines().skip(1) {
        log::info!("{}", line);
    }

    let config = {
        log::info!("Loading configuration");

        config::Config::load()
            .await
            .context("Couldn't load configuration from `config.toml`")?
    };

    let db = {
        log::info!("Initializing database (path = {})", config.database.path);

        if config.database.path.contains(":memory:") {
            log::warn!("");
            log::warn!("!! STARTING WITH AN IN-MEMORY DATABASE !!");
            log::warn!("");
            log::warn!("When you restart Janet, she'll forget everything.");
            log::warn!(
                "To get rid of this warning, please change `database.path` to point at a file."
            );
            log::warn!("");
        }

        database::Database::new(config.database)
            .await
            .context("Couldn't initialize database")?
    };

    let gitlab = {
        log::info!("Initializing GitLab client");

        Arc::new(
            gitlab::GitLabClient::init(config.gitlab.client)
                .await
                .context("Couldn't initialize GitLab client")?,
        )
    };

    let cpu = {
        log::info!("Initializing CPU");

        Arc::new(cpu::Cpu::init(db, gitlab.clone()))
    };

    let gitlab_webhook_handler = Arc::new(gitlab::GitLabWebhookHandler::new(
        config.bot.name,
        config.gitlab.webhook_secret,
        gitlab.clone(),
        cpu.clone(),
    ));

    http::init(config.http, gitlab_webhook_handler).await;

    log::info!("Shutting down");

    Ok(())
}
