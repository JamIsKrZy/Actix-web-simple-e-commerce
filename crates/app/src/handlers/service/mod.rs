use actix_web::web::{self, ServiceConfig};


mod product;
mod user;
mod auth;


pub fn scope(cfg: &mut ServiceConfig){
    cfg.service(
        web::scope("/api")
            .configure(auth::public::scope)

            
    );

    #[cfg(feature="dev_env")]
    cfg.service(
        web::scope("/dev")
            .configure(dev::scope)
    );
}


mod dev{
    use actix_session::Session;
    use actix_web::{get, post, web::{self, ServiceConfig}, HttpResponse, Responder};
    use db_core::ctx::Context;

    pub fn scope(cfg: &mut ServiceConfig){
        cfg.service(check)
            .service(debug_payload)
        ;  
    }


    #[get("/check-cookie/{key}")]
    async fn check(
        key: web::Path<String>,
        session: Session
    ) -> impl Responder {

        let contxt = session.get::<Context>(key.as_str());
        
        match contxt {
            Ok(o) => match o {
                Some(ctxt) => HttpResponse::Ok()
                    .body(format!("Have {} and {:?}", ctxt.id, ctxt.role)),
                None => HttpResponse::Ok()
                    .body("Nothing!"),
            },
            Err(_) => HttpResponse::InternalServerError().body("Error Failed deserialize"),
        }

    }


    #[post("/debug")]
    async fn debug_payload(
        payload: String
    ) -> impl Responder {
        dbg!(&payload);
        HttpResponse::Ok().body("")
    }

    #[post("/new/products")]
    async fn add_product(
        
    ) -> impl Responder {
        ""
    }

    #[post("/new/user")]
    async fn add_user(
        
    ) -> impl Responder {
        ""
    }

}

