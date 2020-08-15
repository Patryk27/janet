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
