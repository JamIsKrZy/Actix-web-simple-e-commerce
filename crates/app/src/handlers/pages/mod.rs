use std::{borrow::Cow};

use actix_files::{Files, NamedFile};
use actix_session::Session;
use actix_web::{get, web::{self, ServiceConfig}, HttpResponse, Responder};
use db_core::{ctx::Context, Role};
use extension::auth_jwt::PermittedType;


mod manage_page;


type PermittedRoles = PermittedType<Context>;


pub fn scope(cfg: &mut ServiceConfig){
    cfg
    
    
    .configure(template::scope)
    .service(login_page)

    // admin
    .service(
        web::scope("/admin")
        .wrap(PermittedRoles::new(
            &[Role::Admin],
            "usr_ctx")
        )
        .configure(manage_page::scope)
    )

    // Worker and admin previlages
    .service(
        web::scope("/work")
        .wrap(PermittedRoles::new(
            &[Role::Admin, Role::Worker],
            "usr_ctx")
        )
        
    )

    .service(
        web::scope("/message")
        .wrap(PermittedRoles::new(
            &[Role::Regular, Role::Admin, Role::Worker],
            "usr_ctx")
        )
        
    )
    
    .service(Files::new("/", "static").index_file("home.html"))
    ;
}


#[get("/login")]
async fn login_page() -> Result<impl Responder, crate::Error> {
    Ok(
        NamedFile::open("static/login.html")
        .map_err(|e| 
            crate::Error::External(Cow::Owned(e.to_string()))
        )?
    )
}




mod template{
    use std::borrow::Cow;

    use lib_core::template_format::{ActionItem, UserActionTemplate};

    use actix_files::NamedFile;
    use actix_session::Session;
    use actix_web::{get, web::{self, ServiceConfig}, HttpResponse, Responder};
    use db_core::{ctx::Context, Role};
    use lib_core::user_action_new;

    use crate::{handlers::pages::manage_page, Error};

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
            NamedFile::open_async("static/template/login.html")
            .await
            .map_err(|e| 
                crate::Error::External(Cow::Owned(e.to_string()))
            )?
        )
    }

    #[get("/auth/signup")]
    async fn signup() -> Result<impl Responder, crate::Error> {
        Ok(
            NamedFile::open_async("static/template/signup.html")
            .await
            .map_err(|e| 
                crate::Error::External(Cow::Owned(e.to_string()))
            )?
        )
    }

    /// Returns list of action of the user based role
    /// Admin: [Records]
    /// Worker: [InOrder]
    /// Regular: [Buy, Search, My Cart]
    /// 
    /// Common: [Message]
    #[get("/actions")]
    async fn user_actions(
        session: Session
    ) -> Result<impl Responder, crate::Error> {
        let user = session.get::<Context>("usr_ctx")
            .map_err(|e| Error::External(Cow::Owned(e.to_string())))?;

        match user {
            Some(ctx) => match ctx.role {
                Role::Regular => {
                    let action = user_action_new!(
                        ("Search","/search"),
                        ("Message","/messages"),
                        ("My Cart","/carts"),
                        ("Profile","/profile")
                    );
                    
                    Ok(HttpResponse::Ok().body(action.to_string()))
                },
                Role::Worker => {
                    
                    let action = user_action_new!(
                        ("Schedule","/work/schedule"),
                        ("Records","/work/records"),
                        ("Message","/messages"),
                        ("Profile","/profile"),
                    );
                    
                    Ok(HttpResponse::Ok().body(action.to_string()))
                },
                Role::Admin => {
                    
                    let action = user_action_new!(
                        ("Manage","/admin/manage"),
                        ("Records","/work/records"),
                        ("Message","/messages"),
                        ("Profile","/profile")
                    );
                    
                    Ok(HttpResponse::Ok().body(action.to_string()))
                },
            },
            None => Err(crate::Error::Unauthorized),
        }
    }

    
}   
