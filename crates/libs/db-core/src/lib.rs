use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};

use crate::utils::DbPoolExtract;


mod utils;
mod error;
mod _dev_utils;
pub mod models;
pub mod ctx;


pub use crate::models::user::Role;



pub use error::DbError;

pub struct PostgressDbManager{
    pool: PgPool 
}

impl PostgressDbManager{

    /// Panic: when no environment provided for DATABASE_URL
    pub async fn new(max: u32) -> Self {
        let url = dotenvy::var("DATABASE_URL")
            .expect("Environment variable \"DATABASE_URL\" not found");

        let pool = PgPoolOptions::new()
            .max_connections(max)
            .connect(&url)
            .await
            .expect("Unable to connect to database");

        Self{ pool }
    }

    /// Used for testing 
    pub async fn init_test_connection() -> Self{
        let url = dotenvy::var("TEST_DATABASE_URL")
            .expect("Environment variable \"TEST_DATABASE_URL\" not found");

        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&url)
            .await
            .expect("Unable to connect to database");

        Self{ pool }
    }




}

impl From<PgPool> for PostgressDbManager{
    fn from(value: PgPool) -> Self {
        Self { pool: value }
    }
}

impl DbPoolExtract<Postgres> for PostgressDbManager{
    fn pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
}



