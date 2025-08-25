use std::sync::Arc;

use actix_web::web::{self, ServiceConfig};
use app::{handlers, set_up};
use db_core::PostgressDbManager;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::SecretStore;
use sqlx::PgPool;


#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://admin:password1233@localhost:6500/e_commerce?schema=public"
    )] pool: PgPool,
    #[shuttle_runtime::Secrets] secrets: SecretStore
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static>  {


    // Apply Migrations
    set_up::shuttle::migrate(&pool).await;

    // Seed Super Admin
    let ( _, _,password_hasher ) = set_up::shuttle::from_base64_secrets(&secrets);
    let hasher = Arc::new(password_hasher);
    set_up::seed::seed_super_admin(&pool, &secrets, hasher).await;    

    unsafe {
        std::env::set_var("RUST_LOG", "info,actix_web=info");
    }

    let routes_shuttle = move |cfg: &mut ServiceConfig| {

        let (
            session_cookie, 
            jwt_handler,
            password_hasher 
        ) = set_up::shuttle::from_base64_secrets(&secrets);

        let dm = web::Data::new(PostgressDbManager::from(pool));
        let jwt_handler = web::Data::new(jwt_handler);
        let password_hasher = web::Data::new(password_hasher);


        cfg.service(
                web::scope("")
                    .app_data(dm.clone())
                    .app_data(jwt_handler)
                    .app_data(password_hasher)
                    .wrap(session_cookie)
                    .configure(handlers::scope)
            )
        ;
    };

    Ok(routes_shuttle.into())
}
