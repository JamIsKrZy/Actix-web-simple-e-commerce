use std::pin::Pin;

use actix_web::{dev::Payload, error::ErrorInternalServerError, Error, FromRequest, HttpMessage as _, HttpRequest};
use uuid::Uuid;

use crate::models::user;

/// Depreactated, new context is in lib-core that handles with actix-session 

#[deprecated]
#[derive(Debug, Clone)]
pub struct Ctx{
    who: Uuid,
    role: user::Role
}

impl FromRequest for Ctx{
    type Error = Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>> + Send + 'static>>;

    fn from_request(req: &actix_web::HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let ctx = req.extensions().get::<Ctx>()
                .cloned();

        Box::pin(async move {
            ctx.ok_or_else(|| 
                ErrorInternalServerError("Unable to find request ctx!")
            )
        })
    }
}





