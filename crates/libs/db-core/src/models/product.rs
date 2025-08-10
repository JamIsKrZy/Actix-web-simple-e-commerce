
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::models::QueryResult;




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

    pub fn insert_one(
        
    ) -> QueryResult<i32> {
        todo!()
    }

}

// endregion: --- Bmc