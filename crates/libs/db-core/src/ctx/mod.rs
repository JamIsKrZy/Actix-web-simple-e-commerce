
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{models::user::UserCredential, Role};

#[derive(Debug, Deserialize, Serialize)]
pub struct Context{
    pub id: Uuid,
    pub role: Role
}

impl From<UserCredential> for Context{
    fn from(value: UserCredential) -> Self {
        Self { id: value.id, role: value.role }
    }
}

impl Context{
    pub fn new(id: Uuid, role: Role) -> Self {
        Self { id, role }
    }
}





