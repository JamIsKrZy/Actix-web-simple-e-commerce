use std::borrow::Cow;

use actix_web::{FromRequest, ResponseError};
use derive_more::Display;


#[derive(Debug, Display)]
pub enum Error{
    #[display("Datbase Error")]
    DatabaseError(db_core::Error),

    #[display("Unauthorized route")]
    Unauthorized,

    External(Cow<'static, str>)
}

impl ResponseError for Error{
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}

