use actix_web::{middleware::Logger, web, App, HttpServer};

use app::{config::config, handlers};
use db_core::PostgressDbManager;
use env_logger::Env;

use lib_core::AppPasswordHasher;






#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    

    #[cfg(feature = "dev_env")]
    {
        
        println!("🛠️ Setting Dev Environement...");
        dotenvy::from_filename_override("dev.env").expect("Unable to locate dev environment");
    }

    // Setting Db
    let dm = web::Data::new(
        if cfg!(feature = "dev_env") {
            PostgressDbManager::init_test_connection().await
        } else {
            PostgressDbManager::new(10).await
        }
    );


    let config = config();
    let app_pass_hash =  web::Data::new(AppPasswordHasher::default());
    let key = config.session_key();

    HttpServer::new( move || {
        let session_handler = app::build_session_handler(key.clone());

        App::new()
            .app_data(dm.clone())
            .app_data(app_pass_hash.clone())
            .wrap(session_handler)
            .wrap(Logger::default())
            .configure(handlers::scope)
            
    })
    .bind(config.bind_addr())?
    .workers(config.workers)
    .run()
    .await
}
