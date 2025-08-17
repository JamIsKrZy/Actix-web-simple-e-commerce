pub mod config;
pub mod error;
pub mod handlers;


use actix_session::{config::{CookieContentSecurity, PersistentSession}, storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::{time::Duration, Key};
pub use error::Error;



pub fn build_session_handler(key: Key) -> SessionMiddleware<CookieSessionStore>{
    let cookie_storage = CookieSessionStore::default();
    let duration = Duration::days(2);


    SessionMiddleware::builder(cookie_storage, key)
        .cookie_http_only(true)
        .cookie_same_site(actix_web::cookie::SameSite::Lax)
        .cookie_secure(true)
        .cookie_content_security(CookieContentSecurity::Private)
        .session_lifecycle(PersistentSession::default().session_ttl(duration))
        .build()
}