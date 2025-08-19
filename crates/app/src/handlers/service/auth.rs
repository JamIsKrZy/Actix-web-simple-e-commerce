


pub mod public {
    use std::borrow::Cow;

    use actix_session::Session;
    use actix_web::{get, http::{header, StatusCode}, post, web::{self, Header, ServiceConfig}, HttpResponse, Responder};
    use db_core::{ctx::Context, models::user::{self, Login, RawPassword, SignUpUser}, PostgressDbManager};
    use lib_core::{AppPasswordHasher, Claim, JwtHandler};
    use support_core::password_hasher::{HashError, HashPassword, PasswordHashifier};

    use crate::{handlers::HandlerResult};

    pub fn scope(cfg: &mut ServiceConfig){
        cfg.service(login)
            .service(signup)

            
        ;
    }



    #[post("/login")]
    async fn login(
        info: web::Json<Login<RawPassword>>,
        pass_hash: web::Data<AppPasswordHasher>,
        db: web::Data<PostgressDbManager>,
        jwt: web::Data<JwtHandler>,
        session: Session
    ) -> HandlerResult<HttpResponse>{

        let username = info.username.as_str();
        let dm = db.get_ref();
        let stored_user = user::Bmc::fetch_one_user(username, dm).await
            .map_err(|e| crate::Error::DatabaseError(e))?;



        let hasher = pass_hash.into_inner();
        let _ = info.into_inner()
            .verify_password(hasher, stored_user.password)
            .await
            .map_err(|_| crate::Error::InternalError)?
            .map_err(|e| crate::Error::HashErr(e))?;
        

        let claim = Claim::new()
            .map_err(|_| crate::Error::InternalError)?;

        let claim = jwt.encode(&claim)
            .map_err(|_| crate::Error::InternalError)?;

        let ctx = Context::new(stored_user.id, stored_user.role);

        // Give User session
        let _ = session.insert("usr_ctx", ctx)
            .map_err(|_| crate::Error::InternalError);
        

        Ok(
        HttpResponse::Found()
            .insert_header((header::AUTHORIZATION, format!("Bearer {}", claim)))    
            .append_header(("HX-Redirect", "/"))
            .body("User Granted!")
        )
    }

    #[get("/check")]
    async fn check(
        session: Session
    ) -> impl Responder {

        let contxt = session.get::<Context>("usr_ctx");
        
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


    #[post("/signup")]
    async fn signup(
        info: web::Form<SignUpUser<RawPassword>>,
        pass_hash: web::Data<AppPasswordHasher>,
        db: web::Data<PostgressDbManager>,
        jwt: web::Data<JwtHandler>,
        session: Session
    ) -> HandlerResult<HttpResponse>{

        

        let hasher = pass_hash.clone().into_inner();
        let dm = db.get_ref();

        let hashed_data = info.into_inner()
            .hash_password(hasher)
            .await
            .map_err(|_| crate::Error::InternalError)?  // web::Block Error
            .map_err(|_| crate::Error::InternalError)?; // Failed hash error

        let ctx = user::Bmc::insert(hashed_data, dm)
            .await
            .map_err(|_| crate::Error::ErrorResponse(
                StatusCode::CONFLICT, 
                Cow::Borrowed("User Already exist")
            ))?;
            
        let claim = Claim::new()
            .map_err(|_| crate::Error::InternalError)?;

        let claim = jwt.encode(&claim)
            .map_err(|_| crate::Error::InternalError)?;

        // Give User session
        let _ = session.insert("usr_ctx", ctx)
            .map_err(|_| crate::Error::InternalError);
            
        

        Ok(
            HttpResponse::Created()
            .insert_header((header::AUTHORIZATION, format!("Bearer {}", claim)))    
            .append_header(("HX-Redirect", "/"))
            .body("User Created")
        )
    }
}

pub mod user {

}

pub mod worker {

}

pub mod admin {

}