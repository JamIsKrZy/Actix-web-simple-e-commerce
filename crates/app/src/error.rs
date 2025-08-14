use std::borrow::Cow;

use actix_web::{http::StatusCode, FromRequest, ResponseError};
use derive_more::Display;
use support_core::password_hasher::HashError;


#[derive(Debug, Display)]
pub enum Error{

    // Internal Server
    #[display("Hash Error internal problem")]
    HashErr(HashError),

    #[display("Error Message: ")]
    ErrorResponse(StatusCode, Cow<'static, str>),

    #[display("Datbase Error")]
    DatabaseError(db_core::DbError),

    #[display("Unauthorized route")]
    Unauthorized,

    #[display("Internal Server Problem")]
    InternalError,

    External(Cow<'static, str>)
}

impl ResponseError for Error{
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}

