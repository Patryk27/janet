use crate::gitlab::GitLabClientConfig;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GitLabConfig {
    #[serde(flatten)]
    pub client: GitLabClientConfig,
    pub webhook_secret: String,
}
