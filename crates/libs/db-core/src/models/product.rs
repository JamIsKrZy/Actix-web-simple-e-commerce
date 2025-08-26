use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{
    Postgres, QueryBuilder,
    prelude::{FromRow, Type},
};
use uuid::Uuid;

use crate::{
    models::{Pagination, ProductStatus, QueryFilterBuilder, QueryResult},
    utils::DbPoolExtract,
};

// region:    --- Enum type

// endregion: --- Enum type

// region:    --- Schemas

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct ProductEssential {
    pub name: String,
    pub id: i32,
}

#[derive(Debug, Deserialize)]
pub struct AddProduct {
    pub name: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub stock: i32,
    pub status: ProductStatus,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct ProductDescription {
    pub id: i32,
    pub name: String,
    pub status: ProductStatus,
    pub description: Option<String>,
    pub price: Decimal,
    pub stocks: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProduct {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<Decimal>,
    pub stock: Option<i32>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct ForPublicProductList {
    pub id: i32,
    pub name: String,
    pub price: Decimal,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct ForAdminProductList {
    pub id: i32,
    pub name: String,
    pub status: ProductStatus,
    pub description: String,
    pub price: Decimal,
    pub stocks: i32,
    pub created_by: String,
    pub created_at: NaiveDateTime,

    pub edited_by: Option<String>,
    pub edited_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct PageFilter {
    min_price: Option<i32>,
    max_price: Option<i32>,
    prefix: Option<String>,
}

impl QueryFilterBuilder for PageFilter {
    fn append_query(&self, query: &mut QueryBuilder<Postgres>) {
        if let Some(prefix) = &self.prefix {
            query
                .push("WHERE name ILIKE ")
                .push_bind(format!("{}%", prefix));
        }
        return;
    }
}

// endregion: --- Schemas

// region:    --- Bmc

pub struct Bmc;

impl Bmc {
    pub async fn get_product_by_id(
        id: i32,
        db: &impl DbPoolExtract<Postgres>,
    ) -> QueryResult<Option<ProductDescription>> {
        let product = sqlx::query_as!(
            ProductDescription,
            "SELECT
                id, name, status as \"status:ProductStatus\",
                price, description, stocks
            FROM products
            WHERE id = $1",
            id
        )
        .fetch_optional(db.pool())
        .await
        .map_err(|e| crate::DbError::FailedSelect { log: e.to_string() })?;

        Ok(product)
    }

    pub async fn set_product_status(
        id: i32,
        status: ProductStatus,
        db: &impl DbPoolExtract<Postgres>,
    ) -> QueryResult<()> {
        let _ = sqlx::query!(
            "UPDATE products
            SET status = $1
            WHERE id = $2",
            status as ProductStatus,
            id
        )
        .execute(db.pool())
        .await
        .map_err(|e| crate::DbError::FailedPatch { log: e.to_string() })?;

        Ok(())
    }

    pub async fn essential_list(
        page: Pagination<PageFilter>,
        db: &impl DbPoolExtract<Postgres>,
    ) -> QueryResult<Vec<ProductID>> {
        let mut query = sqlx::QueryBuilder::new(
            "SELECT
                name, id
                FROM products ",
        );

        page.append_query(&mut query);

        let list = query
            .build_query_as::<ProductID>()
            .fetch_all(db.pool())
            .await
            .map_err(|e| crate::DbError::FailedSelect { log: e.to_string() })?;

        Ok(list)
    }

    pub async fn new_product(
        product: AddProduct,
        who: impl AsRef<Uuid>,
        db: &impl DbPoolExtract<Postgres>,
    ) -> QueryResult<()> {
        let AddProduct {
            name,
            description,
            price,
            stock,
            status,
        } = product;

        let _ = sqlx::query!(
            "
            INSERT INTO products(
                name, description, price, status,
                stocks, created_by 
            ) VALUES (
                $1, $2, $3, $4,
                $5, $6
            )",
            name,
            description,
            price,
            status as ProductStatus,
            stock,
            who.as_ref()
        )
        .execute(db.pool())
        .await
        .map_err(|e| crate::DbError::FailedInsert { log: e.to_string() })?;

        Ok(())
    }

    pub async fn delete_one(id: i32, db: &impl DbPoolExtract<Postgres>) -> QueryResult<()> {
        let _ = sqlx::query!(
            "
            DELETE FROM products
            WHERE id = $1",
            id
        )
        .execute(db.pool())
        .await
        .map_err(|e| crate::DbError::FailedDelete { log: e.to_string() })?;

        Ok(())
    }

    pub async fn public_list<T: QueryFilterBuilder>(
        page: Pagination<T>,
        db: &impl DbPoolExtract<Postgres>,
    ) -> QueryResult<Vec<ForPublicProductList>> {
        let mut query = QueryBuilder::new(
            "Select 
                p.id,
                p.name,
                p.price
            FROM products",
        );

        page.append_query(&mut query);

        let list = query
            .build_query_as::<ForPublicProductList>()
            .fetch_all(db.pool())
            .await
            .map_err(|e| crate::DbError::FailedSelect { log: e.to_string() })?;

        Ok(list)
    }

    pub async fn get_full_list<T: QueryFilterBuilder>(
        page: Pagination<T>,
        db: &impl DbPoolExtract<Postgres>,
    ) -> QueryResult<Vec<ForAdminProductList>> {
        let mut query = QueryBuilder::new(
            "SELECT 
                p.id, 
                p.name,
                p.status,
                p.description,
                p.price, 
                p.stocks, 
                uc.username AS created_by, 
                p.created_at, 
                ue.username AS edited_by, 
                p.edited_at
            FROM products AS p
            LEFT JOIN users AS uc ON p.created_by = uc.id
            LEFT JOIN users AS ue ON p.edited_by = ue.id",
        );

        page.append_query(&mut query);

        let list = query
            .build_query_as::<ForAdminProductList>()
            .fetch_all(db.pool())
            .await
            .map_err(|e| crate::DbError::FailedSelect { log: e.to_string() })?;

        Ok(list)
    }
}

// endregion: --- Bmc
