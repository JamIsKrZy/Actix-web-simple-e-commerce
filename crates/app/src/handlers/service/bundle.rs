
pub mod public {

}

pub mod user {

}

pub mod worker {

}

pub mod admin {
    use actix_web::{get, post, web::{self, ServiceConfig}, HttpResponse};
    use db_core::{models::{bundles::{self, NewBundle}, Pagination}, PostgressDbManager};
    use serde_json::json;

    use crate::{bind_scope_handlers, handlers::{service::bundle, HandlerResult}};

    pub fn scope(cfg: &mut ServiceConfig){
        bind_scope_handlers!(cfg, "/bundles",
            create_bundle
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
    ) -> HandlerResult<HttpResponse> {

        let db = db.as_ref();
        let page = page.into_inner();
        let list = bundles::Bmc::get_full_list(page, db)
            .await?;

        Ok(HttpResponse::Ok().json(json!({ 
            "list": list
        })))
    }
}