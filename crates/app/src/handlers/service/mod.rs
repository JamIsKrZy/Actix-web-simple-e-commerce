use actix_web::web::{self, ServiceConfig};


mod product;
mod user;
mod auth;


pub fn scope(cfg: &mut ServiceConfig){
    cfg.service(
        web::scope("/api")
            .configure(auth::public::scope)
            
    )
    ;
}

