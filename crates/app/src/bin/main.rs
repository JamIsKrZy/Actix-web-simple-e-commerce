use actix_web::{middleware::Logger, web, App, HttpServer};

use app::{config, handlers};
use db_core::PostgressDbManager;
use env_logger::Env;

use lib_core::{AppPasswordHasher, JwtHandler};




#[actix_web::main]
async fn main() -> std::io::Result<()> {


    config::config_envrironment();

    let (dm, jwt_handler) = config::app_data_resource().await;

    let tls = config::load_rustls();
    let config = config::config();
    let app_pass_hash =  web::Data::new(AppPasswordHasher::default());
    let key = config.session_key();
    
    

    HttpServer::new( move || {
        let session_handler = app::build_session_handler(key.clone());

        App::new()
            .app_data(jwt_handler.clone())
            .app_data(dm.clone())
            .app_data(app_pass_hash.clone())
            .wrap(session_handler)
            .wrap(Logger::default())
            .configure(handlers::scope)
            
    })
    .bind_rustls_0_23(config.address(), tls)?
    .workers(config.workers)
    .run()
    .await
}
