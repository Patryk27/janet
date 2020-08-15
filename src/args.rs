use std::path::PathBuf;
use structopt::StructOpt;

/// Janet, a GitLab companion bot | https://github.com/Patryk27/janet
#[derive(StructOpt)]
pub struct Args {
    /// Path to the configuration file
    #[structopt(short, long, default_value = "config.toml")]
    pub config: PathBuf,

    /// Handle all webhooks synchronously; useful only for debugging purposes
    /// and tests
    #[structopt(long)]
    pub sync: bool,
}
