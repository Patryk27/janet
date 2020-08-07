use std::path::PathBuf;
use structopt::StructOpt;

/// Janet, a GitLab companion bot
/// https://github.com/Patryk27/janet
#[derive(StructOpt)]
pub struct Args {
    /// Path to the configuration file
    #[structopt(short = "c", long = "config", default_value = "config.toml")]
    pub config: PathBuf,
}
