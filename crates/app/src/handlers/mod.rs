use actix_web::{web::{self, ServiceConfig}, HttpResponse, Responder};


mod service;
mod pages;

pub(in crate::handlers) type HandlerResult = Result<HttpResponse, crate::Error>;

#[derive(Debug)]
pub enum SessionErr{
    FailedToDeserialize,
    MissingToken
}

pub fn scope(cfg: &mut ServiceConfig){

    cfg.default_service(web::to(default_route))
        .configure(service::scope)
        .configure(pages::scope)
        
    ;
}




async fn default_route() -> impl Responder{
    HttpResponse::NotFound().body(include_str!("../../../../static/notFound.html"))
}


#[deprecated]
mod macro_utils{
    #[macro_export]
    macro_rules! define_scopes {
        ($service_config: expr, $handlers:ident $(, $routes:ident)*) => {
            $service_config
                $(.service(routes))*
        };
        ($service_config: expr, $wraps: ident $(, $middleware:ident)*; $(, $extra_routes:ident)*) => {
            $service_config
                $(.wrap($middleware))*
                $(.service(routes))*

        }
        
    }
}


