use actix_web::{
    HttpResponse, Responder,
    web::{self, ServiceConfig},
};
use utoipa_rapidoc::RapiDoc;

mod pages;
pub mod service;

// Return Type used in every service handler
pub(in crate::handlers) type HandlerResult<T: Responder> = Result<T, crate::Error>;

#[derive(Debug)]
pub enum SessionErr {
    FailedToDeserialize,
    MissingToken,
}

pub fn scope(cfg: &mut ServiceConfig) {
    cfg.default_service(web::to(default_route))
        .configure(service::scope);
}

async fn default_route() -> impl Responder {
    HttpResponse::NotFound().body(include_str!("../../../../static/notFound.html"))
}

#[macro_export]
macro_rules! bind_scope_handlers {
    ($scope:literal, $( $handler: expr ),+) => {
        web::scope( $scope )
            $(
                .service( $handler )
            )+
    };
    ( $cfg:expr, $scope:literal, $( $handler: expr ),+) => {
        $cfg.service(
            web::scope( $scope )
            $(
                .service( $handler )
            )+
        )
    };
}

#[macro_export]
macro_rules! bind_handlers {
    ($cfg:expr, $( $handler: expr ),+) => {
        $cfg$(.service( $handler ))+
    };
}

#[deprecated]
mod macro_utils {
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
