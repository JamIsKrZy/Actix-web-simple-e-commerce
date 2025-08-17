use actix_web::web::{self, ServiceConfig};
use db_core::{ctx::Context, Role};
use extension::auth_jwt::PermittedType;


mod product;
mod user;
mod auth;


type PermittedRoles = PermittedType<Context>;

pub fn scope(cfg: &mut ServiceConfig){
    cfg.service(
        web::scope("/api")
            .configure(auth::public::scope)


            //  User Access
            .service(
                web::scope("/user")
                    .wrap(PermittedRoles::new(
                        &[Role::Regular, Role::Worker, Role::Admin], 
                        "usr_ctx"
                    ))
                    .configure(product::user::scope)
            )

            //  Worker Access
            .service(
                web::scope("/worker")
                    .wrap(PermittedRoles::new(
                        &[Role::Worker, Role::Admin], 
                        "usr_ctx"
                    ))
            )

            //  Admin Access
            .service(
                web::scope("/admin")
                    .wrap(PermittedRoles::new(
                        &[Role::Admin], 
                        "usr_ctx"
                    ))
                    .configure(product::admin::scope)
            )
    );

    #[cfg(feature="dev_env")]
    cfg.service(
        web::scope("/dev")
            .configure(dev::scope)
    );
}




pub(in crate::handlers::service) mod util{
    use actix_session::Session;
    use actix_web::error::ErrorInternalServerError;
    use serde::de::DeserializeOwned;

    use crate::handlers::SessionErr;

    

    pub fn get_session_cookie<T>(
        token: &impl AsRef<str>,
        session: Session
    ) -> Result<T, SessionErr> 
    where 
        T: DeserializeOwned
    {
        let extract_data = session.get::<T>(token.as_ref())
            .map_err(|_| ErrorInternalServerError("Unable to Deserialize"));
        
        
        match extract_data {
            Ok(Some(ctx)) => Ok(ctx),
            Ok(None) => Err(SessionErr::MissingToken), 
            Err(_) => Err(SessionErr::FailedToDeserialize),
        }
    }

}



mod dev{
    use actix_session::Session;
    use actix_web::{get, post, web::{self, ServiceConfig}, HttpResponse, Responder};
    use db_core::{ctx::Context, models::user::{self, AddUser, RawPassword}, PostgressDbManager};
    use lib_core::AppPasswordHasher;

    use support_core::password_hasher::HashPassword;

    use crate::handlers::HandlerResult;

    pub fn scope(cfg: &mut ServiceConfig){
        cfg.service(check)
            .service(debug_payload)
            .service(add_user)
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
        user_info: web::Json<AddUser<RawPassword>>,
        md: web::Data<PostgressDbManager>,
        pass_hash: web::Data<AppPasswordHasher>,
    ) -> HandlerResult {
        
        let hasher = pass_hash.into_inner();
        let hashed_user = user_info.into_inner()
            .hash_password(hasher)
            .await
            .map_err(|_| crate::Error::InternalError)?
            .map_err(|e| crate::Error::HashErr(e))?;


        let _ctx = user::Bmc::insert_with_role(hashed_user, md.as_ref())
            .await
            .map_err(|e| crate::Error::DatabaseError(e))?;

        Ok(HttpResponse::Created().body("User Created!"))
    }

}

