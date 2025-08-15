


pub mod public {
    use actix_web::{get, web::{self, ServiceConfig}};

    use crate::handlers::HandlerResult;

    
    fn scope(cfg: &mut ServiceConfig){
        cfg.service(
            web::scope("/product")
            .service(list)
        )
        ;
    }

    #[get("/list")]
    async fn list(

    ) -> HandlerResult {
        todo!()
    }
}

pub mod user {

}

pub mod worker {

}

pub mod admin {

}