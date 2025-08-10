


pub mod public {
    use actix_web::{get, post, web::{self, ServiceConfig}, HttpResponse};
    use db_core::models::user::{Login, RawPassword, SignUpUser};

    use crate::handlers::HandlerResult;

    pub fn scope(cfg: &mut ServiceConfig){
        cfg.service(login)
            .service(signup)
        ;
    }

    #[post("/login")]
    async fn login(
        info: web::Form<Login<RawPassword>>
    ) -> HandlerResult{

        println!("Login Credential: {:?}", info);

        Ok(HttpResponse::NotImplemented().into())
    }


    #[post("/signup")]
    async fn signup(
        info: web::Form<SignUpUser<RawPassword>>
    ) -> HandlerResult{

        println!("SignUp Credential: {:?}", info);

        Ok(HttpResponse::Created().body("User created"))
    }


}

pub mod user {

}

pub mod worker {

}

pub mod admin {

}