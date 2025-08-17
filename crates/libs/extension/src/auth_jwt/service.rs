use std::{future::{ready, Ready}, marker::PhantomData, pin::Pin};

use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, http::Error};




pub struct AuthJwt<JwtAppData>{ phantom: PhantomData<JwtAppData>}
pub struct AuthJwtService<JwtAppData, S>{
    nxt_service: S,
    _phantom: PhantomData<JwtAppData>
}

impl<S,B,JwtAppData> Transform<S, ServiceRequest> for AuthJwt<JwtAppData> where 
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthJwtService<JwtAppData, S>;
    type InitError=();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthJwtService { nxt_service: service, _phantom: PhantomData }))
    }
}


impl<S,B, JwtAppData> Service<ServiceRequest> for AuthJwtService<JwtAppData, S> where 
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(nxt_service);
    
    fn call(&self, req: ServiceRequest) -> Self::Future {
        todo!()
    }

    
}


