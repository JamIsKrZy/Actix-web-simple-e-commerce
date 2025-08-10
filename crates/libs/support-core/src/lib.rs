

pub mod password_hasher;
pub mod jwt;

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