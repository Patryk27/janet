use crate::features::prelude::*;

#[derive(Clone, Debug, FromRow)]
pub struct User {
    /// Internal user id
    pub id: Id<Self>,

    /// GitLab's user id
    pub ext_id: i64,

    /// When the user's row was created
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn ext_id(&self) -> gl::UserId {
        gl::UserId::new(1)
    }
}
