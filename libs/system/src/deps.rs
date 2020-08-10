use lib_database::Database;
use lib_gitlab::GitLabClient;
use std::sync::Arc;

/// Encapsulates all the dependencies that can be used inside the system.
///
/// This struct allows us to minimize the amount of parameters we'd normally
/// have to pass to each function.
pub struct SystemDeps {
    pub db: Database,
    pub gitlab: Arc<GitLabClient>,
}

#[cfg(test)]
pub struct SystemDepsSpy {
    pub gitlab: lib_gitlab::mock::GitLabMockServer,
}

impl SystemDeps {
    #[cfg(test)]
    pub async fn mock() -> (SystemDepsSpy, Arc<Self>) {
        let (gitlab_server, gitlab_client) = GitLabClient::mock().await;

        let spy = SystemDepsSpy {
            gitlab: gitlab_server,
        };

        let deps = Arc::new(Self {
            db: Database::mock().await,
            gitlab: Arc::new(gitlab_client),
        });

        (spy, deps)
    }
}
