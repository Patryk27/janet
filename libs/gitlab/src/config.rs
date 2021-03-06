use serde::Deserialize;
use url::Url;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GitLabConfig {
    pub url: Url,
    pub personal_access_token: String,
}
