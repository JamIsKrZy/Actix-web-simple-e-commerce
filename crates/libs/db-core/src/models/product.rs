
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Postgres};
use uuid::Uuid;

use crate::{models::QueryResult, utils::DbPoolExtract};




// region:    --- Schemas

#[derive(Debug, Deserialize)]
pub struct NewProduct{
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub stock: i32
} 

#[derive(Debug, Deserialize)]
pub struct UpdateProduct{
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<Decimal>,
    pub stock: Option<i32>
}

#[derive(Debug, FromRow, Serialize)]
pub struct ProductList{
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub stock: i32,
    pub created_by: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub edited_by: Option<String>,
    pub edited_at: Option<DateTime<Utc>>
}



// endregion: --- Schemas

// region:    --- Bmc

pub struct Bmc;

impl Bmc{

    pub async fn insert_one(
        product: NewProduct,
        who: impl AsRef<Uuid>,
        db: &impl DbPoolExtract<Postgres> 
    ) -> QueryResult<()> {

        let NewProduct { 
            name, 
            description, 
            price, 
            stock 
        } = product;
        
        let _ = sqlx::query!("
            INSERT INTO products(
                name, description, price, 
                stocks, created_by 
            ) VALUES (
                $1, $2, $3, 
                $4, $5
            )",
            name, description, price, 
            stock, who.as_ref()
        )
        .execute(db.pool())
        .await
        .map_err(|e| 
            crate::DbError::FailedInsert { log: e.to_string() }
        )?;

        Ok(())

    }


    pub async fn get_list(
        
    ) -> QueryResult<()> {

        Ok(())
    }

}

// endregion: --- Bmc