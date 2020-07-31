use crate::gitlab::{Namespace, ProjectId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Project {
    pub id: ProjectId,
    pub namespace: Namespace,
}
