use actix_web::{middleware::Logger, App, HttpServer};

use app::{config::config, handlers};
use env_logger::Env;

use log::{log};






#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    #[cfg(feature = "dev_env")]
    {
        println!("🛠️ Setting Dev Environement...");
        dotenvy::from_filename_override("dev.env").expect("Unable to locate dev environment");
    }

    let config = config();

    HttpServer::new(|| 
        App::new()
            .wrap(Logger::default())
            .configure(handlers::scope)
            
    )
    .bind(config.bind_addr())?
    .workers(config.workers)
    .run()
    .await
}
