use std::{borrow::Cow, sync::OnceLock};

use actix_files::NamedFile;
use actix_web::{get, web::{self, ServiceConfig}};
use lib_core::template_format::ProductPage;

use crate::{bind_handlers, bind_scope_handlers, handlers::HandlerResult};




pub fn scope(cfg: &mut ServiceConfig){
    cfg.service(
        web::scope("/manage")
            .configure(page_scope)
            .service(page)
    );
}

pub fn page_scope(cfg: &mut ServiceConfig){
    bind_scope_handlers!(cfg, "/page",
        product_page
    );
}


#[get("")]
async fn page() -> HandlerResult<NamedFile>{
    Ok(
        NamedFile::open("./static/manage.html")
        .map_err(|e| 
            crate::Error::External(Cow::Owned(e.to_string()))
        )?
    )
}

#[get("/products")]
async fn product_page() -> HandlerResult<&'static str>{
    static PAGE: OnceLock<String> = OnceLock::new();
    Ok(
        PAGE.get_or_init(|| 
            ProductPage.to_string()
        )
        .as_str()
    )
}

#[get("/bundles")]
async fn bundle_page() -> HandlerResult<&'static str>{
    static PAGE: OnceLock<String> = OnceLock::new();
    Ok(PAGE.get_or_init(|| 
        ProductPage.to_string()).as_str()
    )
}

#[get("/services")]
async fn service_page() -> HandlerResult<&'static str>{
    static PAGE: OnceLock<String> = OnceLock::new();
    Ok(PAGE.get_or_init(|| 
        ProductPage.to_string()).as_str()
    )
}


#[get("/users")]
async fn user_page() -> HandlerResult<&'static str>{
    static PAGE: OnceLock<String> = OnceLock::new();
    Ok(PAGE.get_or_init(|| 
        ProductPage.to_string()).as_str()
    )
}



mod template{



}