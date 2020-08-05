#![feature(try_blocks)]
#![feature(type_ascription)]

use anyhow::*;
use std::sync::Arc;
use tokio::try_join;
use utils::spawn_future;

mod config;
mod database;
mod gitlab;
mod http;
mod interface;
mod log;
mod system;
mod utils;

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

    let (system, system_task) = {
        tracing::info!("Initializing system");
        system::System::init(db, gitlab.clone())
    };

    let gitlab_webhook_handler = Arc::new(gitlab::GitLabWebhookHandler::new(
        config.bot.name,
        config.gitlab.webhook_secret,
        gitlab.clone(),
        system.clone(),
    ));

    let http_task = {
        tracing::info!("Initializing HTTP server");
        http::init(config.http, gitlab_webhook_handler)
    };

    let system_task = spawn_future(system_task);
    let http_task = spawn_future(http_task);

    match try_join!(system_task, http_task) {
        Ok(_) => {
            tracing::info!("Shutting down correctly");
            Ok(())
        }

        Err(err) => {
            tracing::error!("Shutting down because of a system error: {:?}", err);
            Err(err)
        }
    }
}
