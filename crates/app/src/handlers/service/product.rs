pub mod public {
    use actix_web::{
        HttpResponse, get,
        web::{self, ServiceConfig},
    };
    use db_core::{
        PostgressDbManager,
        models::{
            Pagination,
            product::{self, PageFilter},
        },
    };
    use extension::extractor::Accepted;
    use lib_core::template_format;
    use serde_json::json;

    use crate::{bind_scope_handlers, handlers::HandlerResult};

    pub fn scope(cfg: &mut ServiceConfig) {
        cfg.service(bind_scope_handlers!(
            "/products",
            display_list,
            select_option_list
        ));
    }

    #[get("/list")]
    pub async fn display_list() -> HandlerResult<HttpResponse> {
        todo!()
    }

    /// This list is primarly used for
    #[get("/essential-list")]
    pub async fn select_option_list(
        page: web::Query<Pagination<PageFilter>>,
        acpt: extension::extractor::Accepted,
        db: web::Data<PostgressDbManager>,
    ) -> HandlerResult<HttpResponse> {
        let page = page.into_inner();
        let db = db.as_ref();
        let list = product::Bmc::essential_list(page, db).await?;

        match acpt {
            Accepted::Json => Ok(HttpResponse::Ok().json(json!(list))),
            Accepted::Html => Ok(HttpResponse::Ok().body(
                template_format::manage_page::util::OptionProductsTemplate::from(list).to_string(),
            )),
        }
    }
}

pub mod user {
    use actix_web::{
        HttpResponse, get,
        web::{self, ServiceConfig},
    };

    use crate::handlers::HandlerResult;

    pub fn scope(cfg: &mut ServiceConfig) {
        cfg.service(web::scope("/products").service(for_you));
    }

    #[get("/foryou")]
    pub async fn for_you() -> HandlerResult<HttpResponse> {
        Ok(HttpResponse::NotImplemented().body("Not Yet Implemented"))
    }
}

pub mod worker {}

pub mod admin {
    use std::sync::OnceLock;

    use actix_web::{
        HttpResponse, delete, get, patch, post,
        web::{self, ServiceConfig},
    };
    use db_core::{
        PostgressDbManager,
        models::{
            Pagination,
            product::{self, AddProduct},
        },
    };
    use extension::extractor::Accepted;
    use lib_core::template_format::{self, manage_page::EmptyListTable};
    use serde_json::json;

    use crate::{bind_scope_handlers, handlers::HandlerResult};

    pub fn scope(cfg: &mut ServiceConfig) {
        cfg.service(bind_scope_handlers!(
            "/products",
            create_product,
            full_detail_list,
            delete_product
        ));
    }

    #[post("/new")]
    async fn create_product(
        info: web::Json<AddProduct>,
        db: web::Data<PostgressDbManager>,
        usr_ctx: extension::extractor::Context,
    ) -> HandlerResult<HttpResponse> {
        let product = info.into_inner();
        let db = db.as_ref();
        product::Bmc::new_product(product, usr_ctx.id, db).await?;

        Ok(HttpResponse::Created().body("Product have been created!"))
    }

    #[delete("/delete/{id}")]
    pub async fn delete_product(
        id: web::Path<(i32,)>,
        db: web::Data<PostgressDbManager>,
    ) -> HandlerResult<HttpResponse> {
        let id = id.into_inner().0;
        let db = db.as_ref();
        product::Bmc::delete_one(id, db).await?;

        Ok(HttpResponse::Ok().body(""))
    }

    #[get("/list")]
    pub async fn full_detail_list(
        page: web::Query<Pagination<()>>,
        acpt: extension::extractor::Accepted,
        db: web::Data<PostgressDbManager>,
    ) -> HandlerResult<HttpResponse> {
        let list = {
            let db = db.get_ref();
            let page = page.into_inner();
            product::Bmc::get_full_list(page, db).await?
        };

        if list.is_empty() {
            static EMPTY_LIST: OnceLock<String> = OnceLock::new();

            return Ok(HttpResponse::Ok().body(
                EMPTY_LIST
                    .get_or_init(|| EmptyListTable::new_with_size(9).to_string())
                    .as_str(),
            ));
        }

        match acpt {
            Accepted::Json => Ok(HttpResponse::Ok().json(json!({
                "success": {
                    "list": list
                }
            }))),
            Accepted::Html => Ok(HttpResponse::Ok()
                .body(template_format::manage_page::ProductList::from(list).to_string())),
        }
    }

    #[patch("/toggle-status/{id}")]
    pub async fn toggle_product_status(id: web::Path<(i32,)>) -> HandlerResult<HttpResponse> {
        Ok(HttpResponse::NotImplemented().body(""))
    }
}
