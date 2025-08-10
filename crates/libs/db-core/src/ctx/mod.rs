
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Role;

#[derive(Debug, Deserialize, Serialize)]
pub struct Context{
    pub id: Uuid,
    pub role: Role
}





