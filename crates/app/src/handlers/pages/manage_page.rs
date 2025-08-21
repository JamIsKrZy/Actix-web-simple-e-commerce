use std::{borrow::Cow, sync::OnceLock};

use actix_files::NamedFile;
use actix_web::{get, web::{self, ServiceConfig}};
use lib_core::template_format::{ManageMetaData, ManageTemplate, ProductPage};

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
        product_page,
        bundle_page
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
    Ok(
        PAGE.get_or_init(|| 
            ManageTemplate{ 
                api_data: ManageMetaData{ 
                    load_list_endp: "/api/admin/bundles/list", 
                    submit_endp: "/api/admin/bundles/new", 
                    delete_endp: "/api/admin/bundles/delete", 
                    search_endp: "/api/admin/bundles/list" 
                }, 
                title: "Bundles", 
                columns: vec![
                        ("Name", 13), ("Status", 5), ("Items", 5), ("Price", 5),
                        ("Created by", 10), ("Created at", 10), ("Edited by", 10), 
                        ("Edited at", 10), ("Actions", 10)
                    ], 
                filters: vec![
                        ("Active", ""), ("Inactive", ""), ("OutOfStock", "")
                    ],   
            }.to_string()
        )
        .as_str()
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