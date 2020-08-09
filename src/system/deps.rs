use crate::database::Database;
use crate::gitlab::GitLabClient;
use std::sync::Arc;

/// Encapsulates all the dependencies that can be used inside the system.
///
/// This struct allows us to minimize the amount of parameters we'd normally
/// have to pass to each function.
pub struct SystemDeps {
    pub db: Database,
    pub gitlab: Arc<GitLabClient>,
}

impl SystemDeps {
    #[cfg(test)]
    pub async fn mock() -> Arc<Self> {
        Arc::new(Self {
            db: Database::mock().await,
            gitlab: Arc::new(GitLabClient::mock()),
        })
    }
}
