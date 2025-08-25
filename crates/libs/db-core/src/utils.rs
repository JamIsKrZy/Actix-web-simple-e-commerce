use std::pin::Pin;

use sqlx::{Database, Pool, Transaction};



pub trait DbPoolExtract<Db: Database>{

    fn pool(&self) -> &Pool<Db>;
    fn transaction(&self) -> Pin<Box<impl Future< Output = Result<Transaction<'static, Db>, sqlx::Error>>>>;
}