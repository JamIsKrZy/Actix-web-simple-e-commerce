use std::time::{Duration, SystemTime, UNIX_EPOCH};

use argon2::Argon2;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use support_core::{jwt::{JwtDecoder, JwtEncoder}, password_hasher::PasswordHashifier};


pub mod template_format;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claim{
    exp: usize
}

impl Claim{
    pub fn new() -> Result<Self, ()>{
        let exp = SystemTime::now()
            .checked_add(Duration::from_secs(2 * 24 * 60 * 60))
            .ok_or(())?
            .duration_since(UNIX_EPOCH)
            .map_err(|_| ())?
            .as_secs() as usize;
            

        Ok(Self { 
            exp
        })
    }
}


pub struct JwtHandler{
    encode_key: EncodingKey,
    header: Header,
    decode_key: DecodingKey,
    validation: Validation
}


impl JwtEncoder for JwtHandler{
    fn fields(&self) -> (&EncodingKey, &Header) {
        (&self.encode_key, &self.header)
    }
}

impl JwtDecoder for JwtHandler{
    fn fields(&self) -> (&DecodingKey, &Validation) {
        (&self.decode_key, &self.validation)
    }
}

impl JwtHandler{
    pub fn new() -> Self{
        todo!()
    }
}

impl Default for JwtHandler{
    fn default() -> Self {
        Self { 
            encode_key: EncodingKey::from_secret("TEST_DEV_KEY".as_bytes()), 
            header: Default::default(), 
            decode_key: DecodingKey::from_secret("TEST_DEV_KEY".as_bytes()), 
            validation: Default::default() 
        }
    }
}

impl JwtHandler {

    pub fn encode<T>(
        &self, 
        claims: &T
    ) -> Result<String, jsonwebtoken::errors::Error>
    where T: Serialize
    {
        jsonwebtoken::encode(&self.header, claims, &self.encode_key)
    }

    pub fn decode<T>(
        &self, 
        token: &str
    ) -> Result<jsonwebtoken::TokenData<T>, jsonwebtoken::errors::Error>
    where T: for<'a> Deserialize<'a>
    {
        jsonwebtoken::decode::<T>(token, &self.decode_key, &self.validation)
    }
}



pub struct AppPasswordHasher{
    argon: Argon2<'static>
}

impl PasswordHashifier for AppPasswordHasher{
    fn argon2_field(&self) -> &(impl argon2::PasswordHasher + argon2::PasswordVerifier) {
        &self.argon
    }
}

impl Default for AppPasswordHasher{
    fn default() -> Self {
        Self { argon: Default::default() }
    }
}

impl AppPasswordHasher {
    pub fn new() -> Self{
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::{AppPasswordHasher};
    use support_core::password_hasher::PasswordHashifier;


    #[test]
    fn default_verify_password(){
        let hasher = AppPasswordHasher::default();

        let password = "@Password123!";

        let hash_info = hasher.hash_password(password.as_bytes())
            .expect("Failed to hash");

        let _ = hasher.verify_password(
            password.as_bytes(), 
            hash_info
        ).expect("Failed password varify!");

    }
}