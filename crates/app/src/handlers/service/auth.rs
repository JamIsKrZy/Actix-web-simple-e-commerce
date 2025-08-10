


pub mod public {
    use std::borrow::Cow;

    use actix_session::Session;
    use actix_web::{get, http::StatusCode, post, web::{self, ServiceConfig}, HttpResponse};
    use db_core::{models::user::{self, Login, RawPassword, SignUpUser}, PostgressDbManager};
    use lib_core::AppPasswordHasher;
    use support_core::password_hasher::HashPassword;

    use crate::{handlers::HandlerResult};

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
        info: web::Form<SignUpUser<RawPassword>>,
        pass_hash: web::Data<AppPasswordHasher>,
        db: web::Data<PostgressDbManager>,
        session: Session
    ) -> HandlerResult{
        let hasher = pass_hash.clone().into_inner();
        let dm = db.get_ref();

        let hashed_data = info.into_inner()
            .hash_password(hasher)
            .await
            .map_err(|_| crate::Error::InternalError)?  // web::Block Error
            .map_err(|_| crate::Error::InternalError)?; // Failed hash error

        let ctx = user::Bmc::insert(hashed_data, dm).await
            .map_err(|_| crate::Error::ErrorResponse(
                StatusCode::CONFLICT, 
                Cow::Borrowed("User Already exist")
            ))?;
        
        // Give User session
        session.insert("usr_ctx", ctx);
        

        Ok(HttpResponse::Created().body("User Created"))
    }
}

pub mod user {

}

pub mod worker {

}

pub mod admin {

}