use std::borrow::Cow;

use actix_web::{http::StatusCode, ResponseError};
use derive_more::Display;
use support_core::password_hasher::HashError;

use crate::handlers::SessionErr;


#[derive(Debug, Display)]
pub enum Error{

    #[display("Unable to read Cookie")]
    CookieError(SessionErr),

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

impl From<SessionErr> for Error{
    fn from(value: SessionErr) -> Self {
        Self::CookieError(value)
    }
}

impl From<db_core::DbError> for Error{
    fn from(value: db_core::DbError ) -> Self {
        Self::DatabaseError(value)
    }
}

