use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};






pub trait JwtEncoder{
    fn fields(&self) -> (&EncodingKey, &Header);
}

pub trait JwtDecoder{
    fn fields(&self) -> (&DecodingKey, &Validation);
}