use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GitLabConfig {
    #[serde(flatten)]
    pub client: lib_gitlab::GitLabConfig,
    pub webhook_secret: String,
}
