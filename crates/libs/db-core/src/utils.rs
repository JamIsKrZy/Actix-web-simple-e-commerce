use sqlx::{Database, Pool, Transaction};



pub trait DbPoolExtract<Db: Database>{

    fn pool(&self) -> &Pool<Db>;
    // async fn transaction(&self) -> &Transaction<Db>;
}