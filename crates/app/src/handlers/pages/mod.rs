use std::{borrow::Cow};

use actix_files::{Files, NamedFile};
use actix_web::{get, web::{ServiceConfig}, HttpResponse, Responder};




pub fn scope(cfg: &mut ServiceConfig){
    cfg
    
    
    .configure(template::scope)
    .service(login)
    .service(Files::new("/", "./static").index_file("home.html"))
        
    ;
}


#[get("/login")]
async fn login() -> Result<impl Responder, crate::Error> {
    Ok(
        NamedFile::open("./static/login.html")
        .map_err(|e| 
            crate::Error::External(Cow::Owned(e.to_string()))
        )?
    )
}

#[get("/order/{id}")]
async fn order_product() -> impl Responder {
    HttpResponse::NotImplemented()
}

#[get("/book/{id}")]
async fn book_service() -> impl Responder {
    HttpResponse::NotImplemented()
}


mod template{
    use std::borrow::Cow;

    use actix_files::NamedFile;
    use actix_session::Session;
    use actix_web::{get, post, web::{self, ServiceConfig}, Responder};
    use db_core::Role;
    use lib_core::Context;

    use crate::Error;

    pub fn scope(cfg: &mut ServiceConfig){
        cfg.service(
            web::scope("/template")
                .service(user_actions)
                .service(login)
                .service(signup)
        )

        ;
    }

    #[get("/auth/login")]
    async fn login() -> Result<impl Responder, crate::Error> {
        Ok(
            NamedFile::open("./static/template/login.html")
            .map_err(|e| 
                crate::Error::External(Cow::Owned(e.to_string()))
            )?
        )
    }

    #[get("/auth/signup")]
    async fn signup() -> Result<impl Responder, crate::Error> {
        Ok(
            NamedFile::open("./static/template/signup.html")
            .map_err(|e| 
                crate::Error::External(Cow::Owned(e.to_string()))
            )?
        )
    }

    /// Returns list of action of the user based role
    /// Admin: [Products, Service, Control, Records]
    /// Worker: [InOrder]
    /// Regular: [Buy, Search, My Cart]
    /// 
    /// Common: [Message]
    #[get("/actions")]
    async fn user_actions(
        session: Session
    ) -> Result<impl Responder, crate::Error> {
        let user = session.get::<Context>("us_ctx")
            .map_err(|e| Error::External(Cow::Owned(e.to_string())))?;

        match user {
            Some(ctx) => match ctx.role {
                Role::Regular => Ok(""),
                Role::Worker => Ok(""),
                Role::Admin => Ok(""),
            },
            None => Err(crate::Error::Unauthorized),
        }
    }

    
}   
