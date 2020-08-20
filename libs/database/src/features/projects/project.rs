use crate::features::prelude::*;

#[derive(Clone, Debug, FromRow)]
pub struct Project {
    /// Internal id
    pub id: Id<Self>,

    /// GitLab's id
    pub ext_id: i64,

    /// When the model was created in the database
    pub created_at: DateTime<Utc>,
}

impl Project {
    pub fn ext_id(&self) -> gl::ProjectId {
        gl::ProjectId::new(self.ext_id as _)
    }
}
