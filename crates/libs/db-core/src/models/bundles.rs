// region:    --- Schemas

use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::{Postgres, Transaction, prelude::FromRow};
use uuid::Uuid;

use utoipa::ToSchema;

use crate::{
    models::{BIND_LIMIT, Pagination, ProductStatus, QueryResult},
    utils::DbPoolExtract,
};

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct BundleItem {
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde_as]
pub struct NewBundle {
    pub name: String,
    pub price: Decimal,

    #[serde_as(as = "serde_with::VecSkipError<_>")]
    pub items: Vec<BundleItem>,

    #[serde(default)]
    pub status: ProductStatus,
}

#[derive(Debug, FromRow, Serialize, Deserialize, ToSchema)]
pub struct BundleItemDetail {
    pub name: String,
    pub quantity: i32,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct ForAdminBundleList {
    pub id: i32,
    pub name: String,
    pub price: Decimal,
    pub status: ProductStatus,

    #[sqlx(skip)]
    pub items: Option<Vec<BundleItemDetail>>,
    pub created_by: String,
    pub created_at: NaiveDateTime,
    pub edited_by: Option<String>,
    pub edited_at: Option<NaiveDateTime>,
}

// endregion: --- Schemas

// region:    --- Bmc

pub struct Bmc;

impl Bmc {
    pub async fn new_bundle(
        who: Uuid,
        bundle: NewBundle,
        db: &impl DbPoolExtract<Postgres>,
    ) -> QueryResult<()> {
        let mut trans = db
            .transaction()
            .await
            .map_err(|_| crate::DbError::InitTransactionErr)?;

        let NewBundle {
            name,
            price,
            items,
            status,
        } = bundle;

        // Create an instance of the bundle
        // so items can reference to the bundle
        let bundle_id = sqlx::query_scalar!(
            "
            INSERT INTO bundles(
                name, price, status, 
                created_by
            ) 
            VALUES(
                $1, $2, $3,
                $4
            )
            RETURNING id",
            name,
            price,
            status as ProductStatus,
            who
        )
        .fetch_one(&mut *trans)
        .await
        .map_err(|e| crate::DbError::FailedInsert { log: e.to_string() })?;

        Self::insert_items_no_edit_record(bundle_id, items, &mut trans).await?;

        trans
            .commit()
            .await
            .map_err(|_| crate::DbError::TransactionCommitErr)?;

        Ok(())
    }

    async fn insert_items_no_edit_record(
        bundle_id: i32,
        items: impl AsRef<[BundleItem]>,
        db: &mut Transaction<'static, Postgres>,
    ) -> QueryResult<()> {
        let items = items.as_ref();
        let max_rows = BIND_LIMIT / 3; // 3 columns per row
        let items = &items[..items.len().min(max_rows)];

        let mut query = sqlx::QueryBuilder::new(
            "INSERT INTO bundle_items(
                bundle_id, product_id, quantity
            )",
        );

        query.push_values(items, |mut q, i| {
            q.push_bind(bundle_id)
                .push_bind(i.product_id)
                .push_bind(i.quantity);
        });

        query
            .build()
            .execute(&mut **db)
            .await
            .map(|_| ())
            .map_err(|e| crate::DbError::FailedInsert { log: e.to_string() })
    }

    pub async fn insert_items(
        who: Uuid,
        bundle_id: i32,
        items: impl AsRef<[BundleItem]>,
        db: &impl DbPoolExtract<Postgres>,
    ) -> QueryResult<()> {
        Ok(())
    }

    pub async fn remove_items(
        who: Uuid,
        bundle_id: i32,
        items_id: impl AsRef<[BundleItem]>,
        db: &impl DbPoolExtract<Postgres>,
    ) -> QueryResult<()> {
        Ok(())
    }

    pub async fn get_full_list(
        page: Pagination<()>,
        db: &impl DbPoolExtract<Postgres>,
    ) -> QueryResult<Vec<ForAdminBundleList>> {
        let mut query = sqlx::QueryBuilder::new(
            "SELECT 
                b.id, 
                b.name, 
                b.price, 
                b.status,
                uc.username AS created_by, 
                b.created_at, 
                ue.username AS edited_by, 
                b.edited_at
            FROM bundles as b
            LEFT JOIN users AS uc ON b.created_by = uc.id
            LEFT JOIN users AS ue ON b.edited_by = ue.id",
        );

        page.append_query(&mut query);

        let mut list = query
            .build_query_as::<ForAdminBundleList>()
            .fetch_all(db.pool())
            .await
            .map_err(|e| crate::DbError::FailedSelect { log: e.to_string() })?;

        for i in list.iter_mut() {
            let item_list = sqlx::query_as!(
                BundleItemDetail,
                "SELECT 
                    p.name ,
                    b.quantity
                FROM bundle_items as b
                LEFT JOIN products AS p ON b.product_id = p.id
                WHERE bundle_id = $1",
                i.id
            )
            .fetch_all(db.pool())
            .await
            .map_err(|e| crate::DbError::FailedSelect { log: e.to_string() })?;

            i.items = Some(item_list);
        }

        Ok(list)
    }
}

// endregion: --- Bmc

