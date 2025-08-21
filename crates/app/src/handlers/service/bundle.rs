
pub mod public {

}

pub mod user {

}

pub mod worker {

}

pub mod admin {
    use actix_web::{get, post, web::{self, ServiceConfig}, HttpResponse};
    use db_core::{models::{bundles::{self, NewBundle}, Pagination}, PostgressDbManager};
    use lib_core::template_format;
    use serde_json::json;

    use crate::{bind_scope_handlers, handlers::{service::bundle, HandlerResult}};

    pub fn scope(cfg: &mut ServiceConfig){
        bind_scope_handlers!(cfg, "/bundles",
            create_bundle,
            list_bundles
        );
    }

    #[post("/new")]
    async fn create_bundle(
        info: web::Json<NewBundle>,
        db: web::Data<PostgressDbManager>,
        user_ctx: extension::extractor::Context
    ) -> HandlerResult<HttpResponse> { 

        let db = db.as_ref();
        let bundle = info.into_inner();
        bundles::Bmc::new_bundle(user_ctx.id, bundle, db).await?;

        Ok(HttpResponse::Created().body(""))
    }

    #[get("/list")]
    async fn list_bundles(
        page: web::Query<Pagination<()>>,
        db: web::Data<PostgressDbManager>,
        acpt: extension::extractor::Accepted,
    ) -> HandlerResult<HttpResponse> {


        let db = db.as_ref();
        let page = page.into_inner();
        let list = bundles::Bmc::get_full_list(page, db)
            .await?;

        match acpt {
            extension::extractor::Accepted::Json => {
                Ok(HttpResponse::Ok().json(json!({ 
                    "list": list
                })))
            },
            extension::extractor::Accepted::Html => {
                Ok(HttpResponse::Ok().body(
                    template_format::BundleList::from(list).to_string()
                ))
            },
        }
        
    }
}