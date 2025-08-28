use actix_session::{SessionMiddleware, storage::CookieSessionStore};

type SessionCookie = SessionMiddleware<CookieSessionStore>;

#[cfg(feature = "shuttle")]
pub mod shuttle {
    use actix_cors::Cors;
    use actix_session::{SessionMiddleware, storage::CookieSessionStore};
    use actix_web::{cookie::Key, http::header, web::head};
    use lib_core::{AppPasswordHasher, JwtHandler};
    use shuttle_runtime::SecretStore;
    use sqlx::PgPool;

    use crate::set_up::SessionCookie;

    pub fn from_base64_secrets(
        secrets: &SecretStore,
    ) -> (SessionCookie, JwtHandler, AppPasswordHasher) {
        let secret_key_str = secrets
            .get("SECRET_KEY")
            .expect("Unable to find \"SECRET_KEY\"");

        (
            init_session(&secret_key_str),
            init_jwt_handler(&secret_key_str),
            init_password_hasher(&secret_key_str),
        )
    }

    pub async fn migrate(pool: &PgPool) {
        let res = sqlx::migrate!("../../migrations").run(pool).await;

        if let Err(e) = res {
            panic!("Migration Err --- ErrType:{:?} - Description {}", e, e);
        } else {
            // Migration Applied!
        }
    }

    pub fn cors(secrets: &SecretStore) -> Cors {
        let mut cors = Cors::default();

        if let Some(allowed_origins) = secrets.get("ALLOWED_ORIGIN") {
            let allowed_origins = allowed_origins.leak();

            for o in allowed_origins.split(",") {
                cors = cors.allowed_origin(o);
                log::info!("Allowed Origin: {}", o);
                println!("Allowed Origin: {}", o);
            }
        } else {
            log::info!("ALLOWED_ORIGIN not found!");
            println!("ALLOWED_ORIGIN not found!");
        }

        cors.allow_any_method().allowed_headers(&[
            header::ACCEPT,
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
        ])
    }

    fn init_session(secret: &str) -> SessionMiddleware<CookieSessionStore> {
        let key_obj = Key::from(secret.as_bytes());
        crate::build_session_handler(key_obj)
    }

    fn init_jwt_handler(secret: &str) -> JwtHandler {
        JwtHandler::from_base64_secret(secret)
    }

    fn init_password_hasher(secret: &str) -> AppPasswordHasher {
        AppPasswordHasher::from_secret(secret)
    }
}

mod build {}

#[cfg(feature = "shuttle")]
pub mod seed {
    use std::sync::Arc;

    use db_core::{
        DbPoolExtract,
        models::{
            self,
            user::{AddUser, RawPassword},
        },
    };
    use lib_core::AppPasswordHasher;
    use shuttle_runtime::SecretStore;
    use sqlx::PgPool;

    use support_core::password_hasher::HashPassword as _;

    struct DbWrap<'a>(pub &'a PgPool);
    impl<'a> DbPoolExtract<sqlx::Postgres> for DbWrap<'a> {
        fn pool(&self) -> &sqlx::Pool<sqlx::Postgres> {
            self.0
        }

        fn transaction(
            &self,
        ) -> std::pin::Pin<
            Box<
                impl Future<Output = Result<sqlx::Transaction<'static, sqlx::Postgres>, sqlx::Error>>,
            >,
        > {
            Box::pin(self.0.begin())
        }
    }

    pub async fn seed_super_admin(
        pool: &PgPool,
        secrets: &SecretStore,
        hasher: Arc<AppPasswordHasher>,
    ) {
        let wrap_db = DbWrap(pool);

        let admin_username = secrets
            .get("SUPER_ADMIN")
            .expect("Super Admin Username Not Found!");

        let super_admin =
            sqlx::query_scalar!("SELECT (id) FROM users WHERE username = $1", admin_username)
                .fetch_optional(pool)
                .await
                .unwrap_or_else(|e| panic!("Failed to query: {}", e.to_string()));

        if super_admin.is_none() {
            let admin_password = secrets
                .get("SUPER_ADMIN_PASSWORD")
                .expect("Super Admin Password Not Found!");

            let admin_info = AddUser::<RawPassword> {
                username: admin_username.clone(),
                email: "JusMeTheGoat@one.com".to_string(),
                first_name: admin_username,
                last_name: "TheAdmin".to_string(),
                phone_no: "09231495876".to_string(),
                location: Some("TheAnonymousAdmin".to_string()),
                password: admin_password,
                role: db_core::Role::Admin,
                _phantom: std::marker::PhantomData,
            };

            let admin = admin_info
                .hash_password(hasher)
                .await
                .expect("Internal Error: web::Block Fail!")
                .expect("Failed to Hash Password");

            let _ = models::user::Bmc::insert_with_role(admin, &wrap_db)
                .await
                .unwrap_or_else(|e| panic!("Failed Query: {:?}", e));
        }
    }
}
