use argon2::Argon2;
use db_core::Role;
use serde::{Deserialize, Serialize};
use support_core::password_hasher::PasswordHashifier;
use uuid::Uuid;



#[derive(Debug, Deserialize, Serialize)]
pub struct Context{
    pub user_id: Uuid,
    pub role: Role
}


pub struct AppPasswordHasher{
    argon: Argon2<'static>
}

impl PasswordHashifier for AppPasswordHasher{
    fn argon2_field(&self) -> &(impl argon2::PasswordHasher + argon2::PasswordVerifier) {
        &self.argon
    }
}
