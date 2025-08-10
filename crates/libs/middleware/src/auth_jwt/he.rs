use std::{future::{ready, Ready}, pin::Pin,};

use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, error::{ErrorInternalServerError, ErrorUnauthorized}, Error, HttpMessage};


trait PermissionIntance{
    type AsPermission: PartialEq + 'static;
    fn permission_ref(&self) -> &Self::AsPermission;
}


struct PermittedType<E: PermissionIntance>{
    permitted: &'static [E::AsPermission]
}

struct PermittedTypeService<E: PermissionIntance,S>{
    permitted: &'static [E::AsPermission],
    nxt_service: S
}

impl<E> PermittedType<E> where 
    E: PermissionIntance
{
    fn new(permitted: &'static [E::AsPermission]) -> Self{
        Self { permitted }
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
    E: PermissionIntance + 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform=PermittedTypeService<E,S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(PermittedTypeService{ 
            permitted: self.permitted, 
            nxt_service: service 
        }))
    }
}

impl<E,S,B> Service<ServiceRequest> for PermittedTypeService<E,S> where 
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
    E: PermissionIntance + 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(nxt_service);

    fn call(&self, req: ServiceRequest) -> Self::Future {

        let binding = req.extensions();
        let extract_data = binding.get::<E>()
            .ok_or(ErrorInternalServerError("PermittedType: Unable to extract data"));
        
        let data = match extract_data {
            Ok(data) => data.permission_ref(),
            Err(e) => return Box::pin(async move {
                Result::Err(e)
            }),
        };
        
        if self.contains(data) {

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

