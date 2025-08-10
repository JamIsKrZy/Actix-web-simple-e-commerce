use std::sync::Arc;

use actix_web::web;




pub enum HashError{
    FailedHash
}

/// Trait for the app_state that contains hasher
use argon2::password_hash::rand_core::OsRng;
use argon2::{ PasswordHasher, PasswordVerifier};
#[cfg(feature="argon2")]
pub trait PasswordHashifier {


    fn argon2_field(&self) -> &(impl PasswordHasher + PasswordVerifier);

    fn hash_password(&self, password: &[u8]) -> Result<String, HashError> {
        use argon2::password_hash::SaltString;

        let salt = SaltString::generate( &mut OsRng);

        self.argon2_field()
            .hash_password(password, &salt)
            .map(|h| h.to_string())
            .map_err(|_|{
                HashError::FailedHash
            })
    }
} 


pub trait GetPassword{
    fn password_bytes(&self) -> &[u8];
}


pub trait HashPassword<H>: GetPassword where 
    Self: Sized + Send + 'static,
    H: PasswordHashifier + Send + Sync + 'static
{
    type Into: Send + 'static;

    fn to(self, hashed_password: String) -> Self::Into;


    /// Returns a future that is contained web::block as it
    /// is a blocking operation 
    fn hash_password(
        self, 
        hash_manager: Arc<H>
    ) -> impl Future<Output = Result<Result<Self::Into, HashError>, actix_web::error::BlockingError>>{
        web::block(move || {
            let password = self.password_bytes();

            let hashed_pass = hash_manager.hash_password(password)?;
            Ok(self.to(hashed_pass))
        })
    }
}