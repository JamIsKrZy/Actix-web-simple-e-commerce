pub mod config;
pub mod error;
pub mod handlers;
pub mod set_up;

use actix_session::{
    SessionMiddleware,
    config::{CookieContentSecurity, PersistentSession},
    storage::CookieSessionStore,
};
use actix_web::cookie::{Key, SameSite, time::Duration};
pub use error::Error;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(info(description = "An e-commerce-service backend Api"))]
pub struct ApiDoc;

pub fn build_session_handler(key: Key) -> SessionMiddleware<CookieSessionStore> {
    let cookie_storage = CookieSessionStore::default();
    let duration = Duration::days(1);

    // detect environment (you can also use Shuttle's env var or your own)
    let secure = std::env::var("APP_ENV")
        .map(|v| !(v == "dev"))
        .unwrap_or(true);

    SessionMiddleware::builder(cookie_storage, key)
        .cookie_http_only(true)
        .cookie_same_site(SameSite::Lax)
        .cookie_secure(secure) // ✅ false in local dev, true in prod
        .cookie_content_security(CookieContentSecurity::Private)
        .session_lifecycle(PersistentSession::default().session_ttl(duration))
        .build()
}
