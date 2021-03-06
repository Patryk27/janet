#![feature(try_blocks)]

use anyhow::*;
use std::sync::Arc;
use structopt::StructOpt;
use tokio::try_join;

mod args;
mod config;
mod gitlab;
mod http;
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
    let args: args::Args = StructOpt::from_args();

    let config = {
        config::Config::load(&args.config).await.with_context(|| {
            format!(
                "Couldn't load configuration from `{}`",
                args.config.display()
            )
        })?
    };

    log::init(config.log).context("Couldn't initialize log")?;

    for line in LOGO.lines().skip(1) {
        tracing::info!("{}", line);
    }

    let db = {
        tracing::info!("Initializing database (path = {})", config.database.path);

        lib_database::Database::new(config.database)
            .await
            .context("Couldn't initialize database")?
    };

    let gitlab = {
        tracing::info!("Initializing GitLab client");

        Arc::new(
            lib_gitlab::GitLabClient::init(config.gitlab.client)
                .await
                .context("Couldn't initialize GitLab client")?,
        )
    };

    let (system, system_task) = {
        tracing::info!("Initializing system");
        lib_system::System::init(args.sync, db, gitlab.clone())
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
