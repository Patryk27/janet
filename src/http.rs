pub use self::config::*;

use crate::cpu::Cpu;
use std::sync::Arc;
use warp::Filter;

mod config;
mod endpoints;

pub async fn init(config: HttpConfig, cpu: Arc<Cpu>) {
    log::trace!("init()");

    let router = endpoints::health().or(endpoints::handle_gitlab_webhook(cpu));

    log::info!("Starting server at: {}", config.addr);

    warp::serve(router).run(config.addr).await;
}
