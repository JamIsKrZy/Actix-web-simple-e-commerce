use argon2::Argon2;
use db_core::Role;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use support_core::{jwt::{JwtDecoder, JwtEncoder}, password_hasher::PasswordHashifier};
use uuid::Uuid;



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
