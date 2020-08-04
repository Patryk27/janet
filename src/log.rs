pub use self::config::LogConfig;

mod config;

use anyhow::*;
use tracing::Level;
use tracing_gelf::Logger;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::EnvFilter;

pub fn init(config: LogConfig) -> Result<()> {
    let env_filter = EnvFilter::default()
        .add_directive("hyper=error".parse()?)
        .add_directive("warp=error".parse()?)
        .add_directive(LevelFilter::TRACE.into());

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_env_filter(env_filter)
        .finish();

    if let Some(graylog_address) = config.graylog {
        let task = Logger::builder()
            .init_tcp_with_subscriber(graylog_address, subscriber)
            .unwrap();

        tokio::spawn(task);
    } else {
        tracing::subscriber::set_global_default(subscriber)?;
    }

    Ok(())
}
