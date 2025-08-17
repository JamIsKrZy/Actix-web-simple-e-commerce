

pub mod password_hasher;
pub mod jwt;

// Used for interface to communicate with middleware for 
// type validation, expecially on sceniorous where certain roles permitted
pub trait PermissionIntance{
    type AsPermission: PartialEq + 'static;
    fn permission_ref(&self) -> &Self::AsPermission;
}


pub mod auth_jwt{
    use std::marker::PhantomData;

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Claim<T>{
        ctx: T,
        iat: i32,
        exp: i32,
    }

    impl<T> Claim<T>{
        pub fn into_inner(self) -> T {
            self.ctx
        }
    }

    pub trait TypeVerify{
        type Type: PartialEq;
    }

    pub trait ClaimCtx{
        type Ctx: Serialize + for<'a> Deserialize<'a>;
    }

}