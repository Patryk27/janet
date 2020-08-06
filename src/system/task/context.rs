use crate::database::Database;
use crate::gitlab::GitLabClient;
use std::sync::Arc;

// TODO could use a little renaming
pub struct TaskContext {
    pub db: Database,
    pub gitlab: Arc<GitLabClient>,
}

impl TaskContext {
    #[cfg(test)]
    pub async fn mock() -> Arc<Self> {
        Arc::new(Self {
            db: Database::mock().await,
            gitlab: Arc::new(GitLabClient::mock()),
        })
    }
}
