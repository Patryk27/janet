use crate::{Namespace, ProjectId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Project {
    pub id: ProjectId,
    pub namespace: Namespace,
}
