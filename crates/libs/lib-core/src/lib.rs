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
    pub fn from_base64_secret(secret: &str) -> Self{
        let algo = jsonwebtoken::Algorithm::HS256;
        Self { 
            encode_key: EncodingKey::from_base64_secret(secret)
                .expect("Encoding Key: Invalid secret key!"), 
            header: Header::new(algo), 
            decode_key: DecodingKey::from_base64_secret(secret)
                .expect("Decoding Key: Invalid secret key!"), 
            validation: Validation::new(algo) 
        }
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
    argon: Argon2<'static>,
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
    pub fn from_secret(secret: &str) -> Self{
        let alg = argon2::Algorithm::Argon2id;
        let ver = argon2::Version::V0x13;
        let params = argon2::Params::new(
            32768, 
            2, 
            2, 
            None
        ).expect("AppPasswordHasher: Invalid Params"); 
        
        let leak_secret: &'static str = Box::leak(secret.to_string().into_boxed_str());
        
        Self {
            argon: Argon2::new_with_secret(
                leak_secret.as_bytes(), 
                alg, 
                ver, 
                params
            ).expect("AppPasswordHasher: Failed to create Argon"),
        }
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