
use serde::{Deserialize, Serialize};
use support_core::PermissionIntance;
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

impl PermissionIntance for Context{
    type AsPermission = Role;

    fn permission_ref(&self) -> &Self::AsPermission {
        &self.role
    }
}

impl Context{
    pub fn new(id: Uuid, role: Role) -> Self {
        Self { id, role }
    }
}





