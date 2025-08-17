


pub mod public {
    use actix_web::{get, web::{self, ServiceConfig}};

    use crate::handlers::HandlerResult;

    
    pub fn scope(cfg: &mut ServiceConfig){
        cfg.service(
            web::scope("/products")
            .service(list)
        )
        ;
    }

    #[get("/list")]
    async fn list(

    ) -> HandlerResult {
        todo!()
    }
}

pub mod user {
    use actix_web::{get, web::{self, ServiceConfig}, HttpResponse};

    use crate::handlers::HandlerResult;


    pub fn scope(cfg: &mut ServiceConfig){
        cfg.service(
            web::scope("/products")
            .service(for_you)
        )
        ;
    }

    #[get("/foryou")]
    async fn for_you() -> HandlerResult<> {
        Ok(HttpResponse::NotImplemented().body("Not Yet Implemented"))
    }
}

pub mod worker {

}

pub mod admin {
    use actix_web::{get, post, web::{self, ServiceConfig}, HttpResponse};
    use db_core::{models::{product::{self, NewProduct}, Pagination}, PostgressDbManager};
    use extension::extractor::Accepted;
    use lib_core::template_format;
    use serde_json::json;

    use crate::handlers::{HandlerResult};


    pub fn scope(cfg: &mut ServiceConfig){
        cfg.service(
            web::scope("/products")
            .service(create_product)
            .service(full_detail_list)
        );
    }

    #[post("/new")]
    async fn create_product(
        info: web::Json<NewProduct>,
        db: web::Data<PostgressDbManager>,
        usr_ctx: extension::extractor::Context
    ) -> HandlerResult {
        
        let product = info.into_inner();
        let db = db.as_ref();
        let _ = product::Bmc::insert_one(product, usr_ctx.id, db)
            .await?;


        Ok(HttpResponse::Created().body("Product have been created!"))
    }



    #[get("/list")]
    async fn full_detail_list(
        page: web::Query<Pagination<()>>,
        acpt: extension::extractor::Accepted,
        db: web::Data<PostgressDbManager>,
    ) -> HandlerResult {

        let list = {
            let db = db.get_ref();
            let page = page.into_inner();
            product::Bmc::get_list(page, db).await?
        };
        
        match acpt {
            Accepted::Json => {
                Ok(HttpResponse::Ok().json(json!({ 
                    "success": {
                        "list": list
                    }
                })))
            },
            Accepted::Html => {
                Ok(HttpResponse::Ok().body(
                    template_format::ProductList::from(list).to_string()
                ))
            },
        }

    }

}