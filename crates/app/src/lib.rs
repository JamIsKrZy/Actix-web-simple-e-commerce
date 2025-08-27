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
pub use doc::ApiDoc;
pub use error::Error;
use utoipa::OpenApi;

mod doc {
    use crate::handlers::service::{self, auth};
    use utoipa::OpenApi;

    #[derive(OpenApi)]
    #[openapi(
        info(
            title= "ZCom",
            description = "An e-commerce-service backend Api"
        ),
        servers(
            (url="https://zcom-eht8.shuttle.app", description="deployed api hosted in shuttle"),
            (url="http://192.168.244.X:8000", description="If API is available in LAN"),
            (url="http://127.0.0.1:8000", description="If Api is available in localhost network")
        ),
        paths(
            auth::public::login
        ),
        components(
            schemas()
        ),
        tags(
            (name="public", description="Public endpoints that is open the guests and users"),
            (name="user", description="Authenticated Endpoints for valid users"),
            (name="worker", description="Authenticated Endpoints for worker users"),
            (name="admin", description="Authenticated Endpoints for admin users")
        )
    )]
    pub struct ApiDoc;
}

///
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
