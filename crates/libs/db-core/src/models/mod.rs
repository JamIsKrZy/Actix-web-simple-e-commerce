

use serde::{Deserialize, Serialize};
use sqlx::{prelude::Type, Database, Postgres, QueryBuilder};


pub mod user;
pub mod product;
pub mod bundles;


pub(in crate::models) type QueryResult<T> = Result<T, crate::DbError>;


// region:    --- Shared Types

const BIND_LIMIT: usize = 65535;


#[derive(Debug, Type, Clone, Serialize, Deserialize, PartialEq)]
#[sqlx(type_name="product_status", rename_all="PascalCase")]
pub enum ProductStatus{
    Active,
    Inactive
}

impl Default for ProductStatus{
    fn default() -> Self {
        Self::Inactive
    }
}


// endregion: --- Shared Types




pub trait QueryFilterBuilder{
    fn append_query<DB: Database>(&self, query: &mut QueryBuilder<DB>);
}

impl QueryFilterBuilder for () {
    fn append_query<DB: Database>(&self, _: &mut QueryBuilder<DB>) {
        return;
    }
}

#[derive(Debug, Deserialize)]
#[serde(bound(deserialize = "F: Deserialize<'de>"))] // tell Serde the bound for the derived impl
pub struct Pagination<F>
where
    F: QueryFilterBuilder,
{
    pub page: Option<i32>,
    pub limit: Option<i32>,

    #[serde(flatten)]
    pub filter: F,
}

impl<F: QueryFilterBuilder> Pagination<F>{
    pub fn append_query(&self, query: &mut QueryBuilder<Postgres>){
        self.filter.append_query(query);
        query.push(" LIMIT ")
            .push_bind(self.limit.unwrap_or(25))
            .push(" OFFSET ")
            .push_bind(self.page.unwrap_or(0));
    }
}

