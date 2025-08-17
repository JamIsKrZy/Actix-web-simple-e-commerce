use std::{future::{ready, Ready}, pin::Pin,};

use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, error::{ErrorInternalServerError, ErrorUnauthorized}, Error, HttpMessage};
use actix_session::SessionExt;
use serde::{de::DeserializeOwned, Deserialize};
use support_core::PermissionIntance;




pub struct PermittedType<E: PermissionIntance>{
    permitted: &'static [E::AsPermission],
    session_id: &'static str
}

pub struct PermittedTypeService<E: PermissionIntance,S>{
    permitted: &'static [E::AsPermission],
    session_id: &'static str,
    nxt_service: S
}

impl<E> PermittedType<E> where 
    E: PermissionIntance 
{
    pub fn new(permitted: &'static [E::AsPermission], session_id: &'static str) -> Self{
        Self { permitted, session_id }
    }
}

impl<E,S > PermittedTypeService<E,S> where 
    E: PermissionIntance
{
    fn contains(&self, with: &E::AsPermission) -> bool {
        for i in self.permitted{
            if i == with{
                return true;
            }
        }
        return false;
    }
}



impl<E,S,B> Transform<S, ServiceRequest> for PermittedType<E> where 
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
    E: PermissionIntance + for<'de> Deserialize<'de> + DeserializeOwned +'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform=PermittedTypeService<E,S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(PermittedTypeService{ 
            permitted: self.permitted, 
            session_id: self.session_id,
            nxt_service: service 
        }))
    }
}

impl<E,S,B> Service<ServiceRequest> for PermittedTypeService<E,S> where 
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
    E: PermissionIntance + for<'de> Deserialize<'de> + DeserializeOwned +'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(nxt_service);

    fn call(&self, req: ServiceRequest) -> Self::Future {

        let binding = req.get_session();
        let extract_data = binding.get::<E>(self.session_id)
            .map_err(|_| ErrorInternalServerError("Unable to Deserialize"));
        
        
        let ctx = match extract_data {
            Ok(Some(ctx)) => ctx,
            Ok(None) => return Box::pin(async move {
                Result::Err(ErrorInternalServerError("Session key not found!"))
            }), 
            Err(e) => return Box::pin(async move {
                Result::Err(e)
            }),
        };

        let ctx_data = ctx.permission_ref();
        
        if self.contains(ctx_data) {

            drop(binding);
            let service_call = self.nxt_service.call(req);
            Box::pin(async move {
                service_call.await
            })
        } else  {
            Box::pin(async move{
                Result::Err(ErrorUnauthorized("Unauthorized: User is not permitted!"))
            })
        }
    }
}

