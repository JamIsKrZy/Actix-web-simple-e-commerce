use std::future::{Ready, ready};

use actix_session::SessionExt;
use actix_web::{Error, FromRequest, error::ErrorInternalServerError};
use db_core::{Role, models::user::UserCredential};

use serde::{Deserialize, Serialize};
use support_core::PermissionIntance;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Context {
    pub id: Uuid,
    pub role: Role,
}

impl From<UserCredential> for Context {
    fn from(value: UserCredential) -> Self {
        Self {
            id: value.id,
            role: value.role,
        }
    }
}

impl PermissionIntance for Context {
    type AsPermission = Role;

    fn permission_ref(&self) -> &Self::AsPermission {
        &self.role
    }
}

impl Context {
    pub fn new(id: Uuid, role: Role) -> Self {
        Self { id, role }
    }
}

impl FromRequest for Context {
    type Error = Error;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let session = req.get_session().get::<Self>("usr_ctx");

        let Ok(ser) = session else {
            return ready(Err(ErrorInternalServerError(
                "Fail to Deserialize Session!",
            )));
        };

        if let Some(ctxt) = ser {
            ready(Ok(ctxt))
        } else {
            ready(Err(ErrorInternalServerError("Misssing Session Error!")))
        }
    }
}

pub enum Accepted {
    Json,
    Html,
}

impl Default for Accepted {
    fn default() -> Self {
        Self::Json
    }
}

impl FromRequest for Accepted {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let accepted = req
            .headers()
            .get("Accept")
            .map(|v| v.to_str().unwrap_or(""))
            .unwrap_or("");

        let t = if accepted.contains("application/json") {
            Self::Json
        } else if accepted.contains("text/html") {
            Self::Html
        } else {
            Self::default()
        };

        ready(Ok(t))
    }
}

