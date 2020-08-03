use anyhow::Result;
use tracing::Level;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::EnvFilter;

pub fn init() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        // .json() TODO
        .with_max_level(Level::TRACE)
        .with_env_filter(
            EnvFilter::default()
                .add_directive("hyper=error".parse()?)
                .add_directive("warp=error".parse()?)
                .add_directive(LevelFilter::TRACE.into()),
        )
        .finish();

    tracing::subscriber::set_global_default(subscriber).map_err(Into::into)
}
